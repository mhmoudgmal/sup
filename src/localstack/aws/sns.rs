use std::error::Error;

use colored::*;
use log::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_SNS_ENDPOINT: &str = "http://localhost:4575";

pub async fn wait_for_it() -> Result<(), Box<dyn Error>> {
    let mut ready: bool = false;

    while !ready {
        warn!("waiting for localstack's sns to be ready..");

        let cmd = Command::new("aws")
            .arg("sns")
            .arg("list-topics")
            .args(&["--endpoint-url", LOCALSTACK_SNS_ENDPOINT])
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
        AWSService::SNS { topic } => {
            info!("creating sns topic '{}'", name.yellow());

            Command::new("aws")
                .arg("sns")
                .arg("create-topic")
                .args(&["--name", &topic])
                .args(&["--endpoint-url", LOCALSTACK_SNS_ENDPOINT])
                .status()
                .await
                .expect("failed to create sns topic");
        }

        _ => (),
    }

    println!("\n")
}
