use std::fs::metadata;
use std::path::Path;

use colored::*;
use log::*;
use tokio::process::Command;

use crate::ext::rust::PathExt;
use crate::stack::parser::AWSService;

const LOCALSTACK_S3_ENDPOINT: &str = "http://localhost:4572";

pub async fn deploy((name, service): (String, AWSService)) {
    match service {
        AWSService::S3 {
            bucket,
            recreate,
            files,
        } => {
            info!("deploying s3 service '{}'", name.yellow());

            if recreate {
                Command::new("aws")
                    .arg("s3")
                    .arg("rb")
                    .arg("--force")
                    .arg(format!("s3://{}", bucket))
                    .args(&["--endpoint-url", LOCALSTACK_S3_ENDPOINT])
                    .status()
                    .await
                    .expect("failed to remove s3 bucket");
            }

            Command::new("aws")
                .arg("s3")
                .arg("mb")
                .arg(format!("s3://{}", bucket))
                .args(&["--endpoint-url", LOCALSTACK_S3_ENDPOINT])
                .status()
                .await
                .expect("failed to create s3 bucket");

            for file in files {
                if Path::new(&file).does_not_exist() {
                    warn!("file '{}' does not exist, skip..", file.yellow());
                    continue;
                }

                let cmd = if metadata(&file).unwrap().is_dir() {
                    "sync"
                } else {
                    "cp"
                };

                Command::new("aws")
                    .arg("s3")
                    .arg(cmd)
                    .arg(&file)
                    .arg(format!("s3://{}/{}", bucket, file))
                    .args(&["--endpoint-url", LOCALSTACK_S3_ENDPOINT])
                    .status()
                    .await
                    .expect("failed to copy/sync file to s3 bucket");
            }
        }

        _ => (),
    }
}
