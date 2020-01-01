pub mod parser;

use crate::localstack;
use parser::Stack;

pub async fn up(stack: Stack) -> Result<(), Box<dyn std::error::Error>> {
    match localstack::is_running().await {
        Ok(true) => {
            match localstack::running_version().await {
                Ok(running_version) => if running_version == stack.localstack_version {
                    deploy(stack);
                } else {
                    localstack::stop().await?;
                    localstack::remove().await?;
                    localstack::start(&stack.localstack_version).await?;

                    deploy(stack);
                },
                Err(_) => println!("Error??"),
            }

        },

        Ok(false) => {
            localstack::remove().await?;
            localstack::start(&stack.localstack_version).await?;

            deploy(stack);
        },

        Err(_) => println!("Error??"),
    }

    Ok(())
}

fn deploy (stack: Stack) {
    for (name, opts) in stack.services {
        match opts {
            parser::Service::Apigateway { .. } => localstack::aws::apigateway::deploy((name, opts)),
            parser::Service::Dynamo { .. } => localstack::aws::dynamo::deploy((name, opts)),
            parser::Service::Kinesis { .. } => localstack::aws::kinesis::deploy((name, opts)),
            parser::Service::Lambda { .. } => localstack::aws::lambda::deploy((name, opts)),
            parser::Service::S3 { .. } => localstack::aws::s3::deploy((name, opts)),
        }
    }
}
