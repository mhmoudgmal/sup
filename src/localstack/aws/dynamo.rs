use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_DYNAMODB_ENDPOINT: &str = "http://localhost:4569";

pub async fn deploy((name, _service): (String, AWSService)) {
    info!("\ndeploying dynamodb service '{}'\n", name.yellow());

    Command::new("aws")
        .arg("dynamodb")
        // TODO - add the arguments to create the dynamodb table
        .args(&["--endpoint-url", LOCALSTACK_DYNAMODB_ENDPOINT])
        .status()
        .await
        .expect("failed to create dynamodb table");
}
