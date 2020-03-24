pub mod parser;

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::str;

use colored::*;
use log::*;

use crate::localstack;
use crate::stack::parser::{LocalstackConfig, Service, Stack};

const DEFAULT_STACK_FILE: &str = "stackfile";
const SUPPORTED_FORMATS: [&str; 2] = ["json", "yaml"];

pub fn find(stackfile: &str, loc: &str) -> Vec<String> {
    let mut stackfiles_found = Vec::new();

    if stackfile == "" {
        for format in SUPPORTED_FORMATS.iter() {
            let file = format!("{}/{}.{}", loc, DEFAULT_STACK_FILE, format);

            if Path::new(&file).exists() {
                stackfiles_found.push(file);
            }
        }
    } else {
        let file = format!("{}/{}", loc, stackfile);

        if Path::new(&file).exists() {
            stackfiles_found.push(file);
        }
    }

    return stackfiles_found;
}

pub async fn up(stack: Stack) -> Result<(), Box<dyn std::error::Error>> {
    async fn recreate(config: &LocalstackConfig) {
        localstack::remove().await;
        info!("starting localstack version: '{}'", config.version.yellow());
        localstack::start(&config).await;
    }

    if stack.localstack_config.recreate {
        warn!("stopping localstack");
        localstack::stop().await;
        recreate(&stack.localstack_config).await;
    }

    info!("checking if localstack is running");
    if localstack::is_running().await? {
        let running_version = localstack::running_version().await?;
        if running_version == stack.localstack_config.version {
            info!("localstack is already running");
        } else {
            warn!(
                "localstack is already running on a differnt version: '{}'",
                running_version.yellow()
            );
            warn!("stopping localstack");
            localstack::stop().await;
            recreate(&stack.localstack_config).await;
        }
    } else {
        recreate(&stack.localstack_config).await;
    }

    deploy(stack);
    Ok(())
}

fn deploy(stack: Stack) {
    for (name, service) in stack.services {
        match service {
            Service { deps, .. } => {
                for (name, dep) in deps {
                    info!(
                        "deploying dependency: '{}' at '{}'",
                        name.yellow(),
                        dep.location.yellow()
                    );

                    let mut stackfiles_found = find(&dep.stackfile, &dep.location);

                    if stackfiles_found.len() > 1 {
                        error!(
                            "more than one stackfile was found for dep '{}' at '{}'",
                            name.yellow(),
                            dep.location.yellow()
                        );
                        for file in stackfiles_found.iter() {
                            info!("found {}", file.yellow());
                        }
                        error!("skipping dep '{}'", name.yellow());
                        continue;
                    }

                    let dep_stackfile = match stackfiles_found.pop() {
                        Some(f) => f,
                        None => {
                            error!(
                                "no stackfile found for dep '{}' at '{}'",
                                name.yellow(),
                                dep.location.yellow()
                            );
                            error!("skipping dep '{}'", name.yellow());
                            continue;
                        }
                    };

                    let stack_format = match Path::new(&dep_stackfile)
                        .extension()
                        .and_then(OsStr::to_str)
                    {
                        Some(ext) => ext,
                        None => "",
                    };

                    let dep_stack = match crate::stack::parser::parse(&dep_stackfile, &stack_format)
                    {
                        Some(stack) => stack,
                        None => continue,
                    };

                    let wd = env::current_dir().expect("failed to get current working dir");

                    env::set_current_dir(dep.location).expect("failed to change dir");
                    deploy(dep_stack);
                    env::set_current_dir(wd).expect("failed to change dir");
                }
            }
        }

        let aws_service = service.variant;
        localstack::aws::deploy((name, aws_service));
    }
}
