use crate::stack::parser::AWSService;

pub fn deploy((name, opts): (String, AWSService)) {
    println!("deploying apigateway {} : {:?}\n", name, opts);
}
