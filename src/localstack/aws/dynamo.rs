use crate::stack::parser::AWSService;

pub fn deploy((name, opts): (String, AWSService)) {
    println!("deploying dynamodb {} : {:?}\n", name, opts);
}
