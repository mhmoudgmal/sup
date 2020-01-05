use crate::stack::parser::AWSService;

pub fn deploy((name, opts): (String, AWSService)) {
    println!("deploying s3 {} : {:?}\n", name, opts);
}
