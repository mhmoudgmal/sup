use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_KINESIS_ENDPOINT: &str = "http://localhost:4568";

pub async fn deploy((name, _service): (String, AWSService)) {
    info!("\ndeploying kinesis service '{}'\n", name.yellow());

    Command::new("aws")
        .arg("kinesis")
        // TODO - add the arguments to create the kinesis stream
        .args(&["--endpoint-url", LOCALSTACK_KINESIS_ENDPOINT])
        .status()
        .await
        .expect("failed to create stream");
}
