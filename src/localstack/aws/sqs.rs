use log::*;

use crate::stack::parser::AWSService;

pub fn deploy((name, opts): (String, AWSService)) {
    info!("\ndeploying s3 {} : {:?}\n", name, opts);
}
