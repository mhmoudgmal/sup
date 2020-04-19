use std::error::Error;

use colored::*;
use log::*;
use tokio::process::Command;

use crate::stack::parser::AWSService;

const LOCALSTACK_APIGATEWAY_ENDPOINT: &str = "http://localhost:4567";

pub async fn wait_for_it() -> Result<(), Box<dyn Error>> {
    let mut ready: bool = false;

    while !ready {
        warn!("waiting for localstack's apigateway to be ready..");

        let cmd = Command::new("aws")
            .arg("apigateway")
            .arg("get-rest-apis")
            .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
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
        AWSService::Apigateway {
            api_name,
            path_part,
            http_method,
            authorization_type,
            integration_type,
            integration_http_method,
            uri,
        } => {
            info!("\ndeploying apigateway '{}'\n", name.yellow());

            Command::new("aws")
                .arg("apigateway")
                // TODO - add the arguments to create the apigateway resource
                .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
                .status()
                .await
                .expect("failed to create apigateway resource");
        }

        _ => (),
    }
}
