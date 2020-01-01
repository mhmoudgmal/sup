use crate::stack::parser::Service;

pub fn deploy((name, opts): (String, Service)) {
    println!("deploying s3 {} : {:?}\n", name, opts);
}
