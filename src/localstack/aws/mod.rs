pub mod apigateway;
pub mod dynamo;
pub mod kinesis;
pub mod lambda;
pub mod s3;
pub mod sns;
pub mod sqs;

use futures::executor::block_on;

use crate::stack::parser::AWSService::*;
use crate::stack::parser::*;

pub fn deploy((name, service): (String, AWSService)) {
    match service {
        Apigateway { .. } => block_on(apigateway::deploy((name, service))),
        Dynamo { .. } => block_on(dynamo::deploy((name, service))),
        Kinesis { .. } => block_on(kinesis::deploy((name, service))),
        Lambda { .. } => block_on(lambda::deploy((name, service))),
        S3 { .. } => block_on(s3::deploy((name, service))),
        SNS { .. } => block_on(sns::deploy((name, service))),
        SQS { .. } => block_on(sqs::deploy((name, service))),
    }
}
