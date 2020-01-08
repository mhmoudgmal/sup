use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_SQS_ENDPOINT: &str = "http://localhost:4576";

pub async fn deploy((name, _service): (String, AWSService)) {
    info!("\ndeploying sqs service '{}'\n", name.yellow());

    Command::new("aws")
        .arg("sqs")
        // TODO - add the arguments to create the sqs topic
        .args(&["--endpoint-url", LOCALSTACK_SQS_ENDPOINT])
        .status()
        .await
        .expect("failed to create sqs queue");
}
