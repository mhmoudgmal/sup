use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_SQS_ENDPOINT: &str = "http://localhost:4576";

pub async fn deploy((name, service): (String, AWSService)) {
    match service {
        AWSService::SQS { queue } => {
            info!("creating sqs queue '{}'", name.yellow());

            Command::new("aws")
                .arg("sqs")
                .arg("create-queue")
                .args(&["--queue-name", &queue])
                .args(&["--endpoint-url", LOCALSTACK_SQS_ENDPOINT])
                .status()
                .await
                .expect("failed to create sqs queue");
        },

        _ => ()
    }

    println!("\n")
}
