use std::ffi::OsStr;
use std::path::Path;
use std::process::exit;

use ext::rust::PathExt;

mod app;
mod ext;
mod localstack;
mod stack;

#[tokio::main]
pub async fn main() {
    let stackfile: String;
    let args = app::match_args();

    match args.value_of("stackfile") {
        Some(f) => {
            if Path::new(&f).does_not_exist() {
                println!("stackfile {} does not exit", f);
                exit(1)
            }

            stackfile = String::from(f);
        }

        None => {
            let mut stackfiles_found = stack::find("", ".");

            if stackfiles_found.len() > 1 {
                println!("more than one stackfile was found in the current directory");
                println!("stackfiles:");
                for file in stackfiles_found.iter() {
                    println!("\t- {}", file);
                }
                exit(1);
            }

            stackfile = match stackfiles_found.pop() {
                Some(f) => f,
                _ => {
                    println!("stackfile does not exist in the current working directory");
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
        Ok(_) => {},
        Err(_) => {},
    }
}
