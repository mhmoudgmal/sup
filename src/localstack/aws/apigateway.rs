use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_APIGATEWAY_ENDPOINT: &str = "http://localhost:4567";

pub async fn deploy((name, _service): (String, AWSService)) {
    info!("\ndeploying apigateway '{}'\n", name.yellow());

    Command::new("aws")
        .arg("apigateway")
        // TODO - add the arguments to create the apigateway resource
        .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
        .status()
        .await
        .expect("failed to create apigateway resource");
}
