use crate::stack::parser::Service;

pub fn deploy((name, opts): (String, Service)) {
    println!("deploying apigateway {} : {:?}\n", name, opts);
}
