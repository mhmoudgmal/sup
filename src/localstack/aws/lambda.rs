use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_LAMBDA_ENDPOINT: &str = "http://localhost:4574";

pub async fn deploy((name, _service): (String, AWSService)) {
    info!("\ndeploying lambda service '{}'\n", name.yellow());

    Command::new("aws")
        .arg("lambda")
        // TODO - add the arguments to create the lambda funcrion
        .args(&["--endpoint-url", LOCALSTACK_LAMBDA_ENDPOINT])
        .status()
        .await
        .expect("failed to create lambda");
}
