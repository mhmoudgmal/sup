use std::error::Error;

use colored::*;
use log::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_SQS_ENDPOINT: &str = "http://localhost:4576";

pub async fn wait_for_it() -> Result<(), Box<dyn Error>> {
    let mut ready: bool = false;

    while !ready {
        warn!("waiting for localstack's sqs to be ready..");

        let cmd = Command::new("aws")
            .arg("sqs")
            .arg("list-queues")
            .args(&["--endpoint-url", LOCALSTACK_SQS_ENDPOINT])
            .status()
            .await?;

        if cmd.success() {
            ready = true;
        }
    }
    Ok(())
}

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
        }

        _ => (),
    }

    println!("\n")
}
