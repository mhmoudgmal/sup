use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::str;

use colored::*;
use log::*;
use serde_json::json;
use tokio::process::Command;

use crate::ext::rust::PathExt;
use crate::stack::parser::AWSService;

const LOCALSTACK_LAMBDA_ENDPOINT: &str = "http://localhost:4574";

pub async fn deploy((name, service): (String, AWSService)) {
    match service {
        AWSService::Lambda {
            runtime,
            handler,
            env_file,
            function_name,
            env_vars,
            files,
            function_path,
        } => {
            info!(
                "deploying lambda service '{}' at '{}'",
                name.yellow(),
                function_path.yellow()
            );

            let wd = env::current_dir().expect("failed to get current working dir");

            env::set_current_dir(function_path).expect("failed to change dir");

            if function_exists(&function_name).await {
                Command::new("aws")
                    .arg("lambda")
                    .arg("delete-function")
                    .args(&["--function-name", &function_name])
                    .args(&["--endpoint-url", LOCALSTACK_LAMBDA_ENDPOINT])
                    .status()
                    .await
                    .expect("failed to delete lambda");
            }

            let zipfile = create_zip(&name, files).await;
            let vars = variables(env_file, env_vars);

            Command::new("aws")
                .arg("lambda")
                .arg("create-function")
                .args(&["--runtime", &runtime])
                .args(&["--handler", &handler])
                .args(&["--environment", &vars])
                .args(&["--function-name", &function_name])
                .args(&["--zip-file", &format!("fileb://{}", zipfile)])
                .args(&["--role", "arn:aws:iam::000000000000:role/lsup"])
                .args(&["--endpoint-url", LOCALSTACK_LAMBDA_ENDPOINT])
                .status()
                .await
                .expect("failed to create lambda");

            info!(
                "lambda function '{}' has been deployed",
                function_name.yellow()
            );
            delete_zip(zipfile).await;

            env::set_current_dir(wd).expect("failed to change dir");
        }

        _ => (),
    }
}

fn variables(env_file: String, env_vars: HashMap<String, String>) -> String {
    // src: https://docs.rs/configurable/0.3.3/src/configurable/env.rs.html#22-38
    // https://github.com/museun/configurable/issues/4
    let mut vars = std::fs::read_to_string(env_file)
        .map(|data| {
            data.lines()
                .filter(|s| !s.starts_with('#'))
                .filter_map(|line| {
                    let mut line = line.splitn(2, '=').map(str::trim);
                    Some((line.next()?.into(), line.next()?.into()))
                })
                .collect()
        })
        .unwrap_or_else(|_| HashMap::new());

    for (k, v) in env_vars {
        vars.insert(k, v);
    }

    return json!({ "Variables": vars }).to_string();
}

async fn function_exists(function_name: &str) -> bool {
    let status = Command::new("aws")
        .arg("lambda")
        .arg("get-function")
        .args(&["--function-name", &function_name])
        .args(&["--endpoint-url", LOCALSTACK_LAMBDA_ENDPOINT])
        .status()
        .await
        .expect("failed to check for function");

    if status.success() {
        return true;
    } else {
        return false;
    }
}

async fn create_zip(name: &str, files: Vec<String>) -> String {
    info!("Create zipfile for ({}) files", files.join(", "));

    // TODO: replace lsup with unique dynamic value
    let zipfile = format!("{}_{}.zip", name, "lsup");

    // create empty zip file
    Command::new("zip")
        .args(&[&zipfile])
        .status()
        .await
        .expect("failed to create zip file");

    // add files and directories to the zip file if they exist
    for file in files {
        if Path::new(&file).does_not_exist() {
            warn!("file '{}' does not exist, skip..", file.yellow());
            continue;
        }

        Command::new("zip")
            .args(&["-ru", &zipfile, &file])
            .status()
            .await
            .expect("failed to add file to the zip file");
    }

    info!("zip file has been created");
    return zipfile;
}

async fn delete_zip(zipfile: String) {
    info!("Cleanup ...");

    Command::new("rm")
        .args(&[&zipfile])
        .status()
        .await
        .expect("failed to delete zip file");
}
