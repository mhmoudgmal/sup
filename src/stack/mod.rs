pub mod parser;

use std::ffi::OsStr;
use std::path::Path;

use log::*;
use colored::*;

use crate::localstack;
use parser::Stack;
use parser::Service;

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
    match localstack::is_running().await {
        Ok(true) => {
            info!("checking if localstack is running");
            match localstack::running_version().await {
                Ok(running_version) => if running_version == stack.localstack_config.version {
                    info!("localstack is already running");
                    deploy(stack);
                } else {
                    warn!("localstack is already running on a differnt version: '{}'", running_version.yellow());
                    warn!("stopping localstack version: '{}'", running_version.yellow());
                    localstack::stop().await;
                    localstack::remove().await;
                    info!("starting localstack version: '{}'", &stack.localstack_config.version.yellow());
                    localstack::start(&stack.localstack_config).await;

                    deploy(stack);
                },
                Err(e) => error!("{}", e),
            }

        },

        Ok(false) => {
            localstack::remove().await;
            info!("starting localstack version: '{}'", &stack.localstack_config.version.yellow());
            localstack::start(&stack.localstack_config).await;

            deploy(stack);
        },

        Err(e) => error!("{}", e),
    }

    Ok(())
}

fn deploy (stack: Stack) {
    for (name, service) in stack.services {
        match &service {
            Service { deps, .. } => {
                for (name, dep) in deps {
                    info!("deploying dependency: '{}' at '{}'", name.yellow(), dep.location.yellow());

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
                        _ => {
                            error!(
                                "no stackfile found for dep '{}' does not exist in '{}'",
                                name.yellow(),
                                dep.location.yellow()
                            );
                            error!("skipping dep '{}'", name.yellow());
                            continue;
                        }
                    };

                    let stack_format = match Path::new(&dep_stackfile).extension().and_then(OsStr::to_str) {
                        Some(ext) => ext,
                        _ => "",
                    };

                    let dep_stack = match crate::stack::parser::parse(&dep_stackfile, &stack_format) {
                        Some(stack) => stack,
                        None => continue,
                    };

                    // TODO: cd to dependency stack location
                    deploy(dep_stack);
                    // TODO: cd back to dependant stack location
                }
            }
        }

        let aws_service = service.variant;
        localstack::aws::deploy((name, aws_service));
    }
}
