use std::ffi::OsStr;
use std::path::Path;
use std::process::exit;

const SUPPORTED_FORMATS: [&str; 2] = ["json", "yaml"];
const DEFAULT_STACK_FILE: &str = "stackfile";

mod app;
mod stack;

fn main() {
    let stackfile: String;
    let args = app::match_args();

    match args.value_of("stackfile") {
        Some(f) => {
            if !Path::new(&f).exists() {
                println!("stackfile {} does not exit", f);
                exit(1)
            }

            stackfile = String::from(f);
        }

        None => {
            let mut stackfiles_found = Vec::new();

            for format in SUPPORTED_FORMATS.iter() {
                let file = format!("{}.{}", DEFAULT_STACK_FILE, format);

                if Path::new(&file).exists() {
                    stackfiles_found.push(file);
                }
            }

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

    let stack = stack::parser::parse(&stackfile, &stack_format);

    match stack::up(stack) {
        _ => {}
    }
}
