use crate::stack::parser::AWSService;

pub fn deploy((name, opts): (String, AWSService)) {
    println!("deploying kinesis {} : {:?}\n", name, opts);
}
