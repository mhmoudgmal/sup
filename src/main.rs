use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::exit;

use colored::*;
use log::*;
use pretty_env_logger;

use ext::rust::PathExt;

mod app;
mod ext;
mod localstack;
mod stack;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    env::set_var("AWS_PAGER", "");
    env::set_var("RUST_LOG", "info");

    let stackfile: String;
    let args = app::match_args();

    match args.value_of("stackfile") {
        Some(f) => {
            if Path::new(&f).does_not_exist() {
                error!("stackfile '{}' does not exit", f.yellow());
                exit(1)
            }

            stackfile = String::from(f);
        }

        None => {
            let mut stackfiles_found = stack::find("", ".");

            if stackfiles_found.len() > 1 {
                error!(
                    "{}",
                    "more than one stackfile was found in the current directory".red()
                );
                for file in stackfiles_found.iter() {
                    info!("found {}", file.yellow());
                }
                exit(1);
            }

            stackfile = match stackfiles_found.pop() {
                Some(f) => f,
                _ => {
                    error!(
                        "{}",
                        "stackfile does not exist in the current working directory".red()
                    );
                    exit(1)
                }
            };
        }
    };

    let stack_format = match Path::new(&stackfile).extension().and_then(OsStr::to_str) {
        Some(ext) => ext,
        _ => "",
    };

    let stack = match stack::parser::parse(&stackfile, &stack_format) {
        Some(stack) => stack,
        None => exit(1),
    };

    match stack::up(stack).await {
        Ok(_) => {}
        Err(err) => error!("{}", err),
    }
}
