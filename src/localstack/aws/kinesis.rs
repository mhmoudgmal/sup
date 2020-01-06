use log::*;

use crate::stack::parser::AWSService;

pub fn deploy((name, opts): (String, AWSService)) {
    info!("\ndeploying kinesis {} : {:?}\n", name, opts);
}
