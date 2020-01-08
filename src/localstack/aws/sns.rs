use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_SNS_ENDPOINT: &str = "http://localhost:4575";

pub async fn deploy((name, _service): (String, AWSService)) {
    info!("\ndeploying sns service '{}'\n", name.yellow());

    Command::new("aws")
        .arg("sns")
        // TODO - add the arguments to create the sns topic
        .args(&["--endpoint-url", LOCALSTACK_SNS_ENDPOINT])
        .status()
        .await
        .expect("failed to create sns topic");
}
