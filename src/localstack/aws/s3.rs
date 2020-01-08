use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_S3_ENDPOINT: &str = "http://localhost:4572";

pub async fn deploy((name, _service): (String, AWSService)) {
    info!("\ndeploying s3 service '{}'\n", name.yellow());

    Command::new("aws")
        .arg("s3")
        // TODO - add the arguments to create the s3 bucket
        .args(&["--endpoint-url", LOCALSTACK_S3_ENDPOINT])
        .status()
        .await
        .expect("failed to create s3 bucket");
}
