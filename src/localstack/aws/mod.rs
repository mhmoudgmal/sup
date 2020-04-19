pub mod apigateway;
pub mod dynamo;
pub mod kinesis;
pub mod lambda;
pub mod s3;
pub mod sns;
pub mod sqs;

use std::error::Error;

use crate::stack::parser::AWSService::*;
use crate::stack::parser::*;

pub async fn deploy((name, service): (String, AWSService)) {
    match service {
        Apigateway { .. } => apigateway::deploy((name, service)).await,
        Dynamo { .. } => dynamo::deploy((name, service)).await,
        Kinesis { .. } => kinesis::deploy((name, service)).await,
        Lambda { .. } => lambda::deploy((name, service)).await,
        S3 { .. } => s3::deploy((name, service)).await,
        SNS { .. } => sns::deploy((name, service)).await,
        SQS { .. } => sqs::deploy((name, service)).await,
    }
}

pub async fn wait_for_it(services: &Vec<String>) -> Result<(), Box<dyn Error>> {
    for service in services {
        match service.as_str() {
            "apigateway" => apigateway::wait_for_it().await?,
            "dynamodb" => dynamo::wait_for_it().await?,
            "kinesis" => kinesis::wait_for_it().await?,
            "lambda" => lambda::wait_for_it().await?,
            "s3" => s3::wait_for_it().await?,
            "sns" => sns::wait_for_it().await?,
            "sqs" => sqs::wait_for_it().await?,
            _ => (),
        };
    }
    Ok(())
}
