use crate::stack::parser::Service;

pub fn deploy((name, opts): (String, Service)) {
    println!("deploying lambda {} : {:?}\n", name, opts);
}
