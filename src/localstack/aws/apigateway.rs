use std::error::Error;

use colored::*;
use log::*;
use std::str;

use serde_json::Value;
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
            .args(&["--region", "us-east-1"])
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
            info!("deploying apigateway '{}'", name.yellow());

            let api_id = create_rest_api(&api_name).await;
            let parent_id = parent_id(&api_id).await;
            let resource_id = create_resource(&path_part, &parent_id, &api_id).await;

            Command::new("aws")
                .arg("apigateway")
                .arg("put-method")
                .args(&["--http-method", &http_method])
                .args(&["--rest-api-id", &api_id])
                .args(&["--resource-id", &resource_id])
                .args(&["--authorization-type", &authorization_type])
                .args(&["--region", "us-east-1"])
                .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
                .status()
                .await
                .expect("failed to update resource for the http_method");

            Command::new("aws")
                .arg("apigateway")
                .arg("put-integration")
                .args(&["--uri", &uri])
                .args(&["--type", &integration_type])
                .args(&["--http-method", &http_method])
                .args(&["--rest-api-id", &api_id])
                .args(&["--resource-id", &resource_id])
                .args(&["--integration-http-method", &integration_http_method])
                .args(&["--region", "us-east-1"])
                .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
                .status()
                .await
                .expect("failed to update the resource for integration info");

            Command::new("aws")
                .arg("apigateway")
                .arg("create-deployment")
                .args(&["--rest-api-id", &api_id])
                .args(&["--stage-name", "sup"])
                .args(&["--region", "us-east-1"])
                .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
                .status()
                .await
                .expect("failed to create deployment");
        }

        _ => (),
    }
}

async fn create_rest_api(api_name: &str) -> String {
    let cmd = Command::new("aws")
        .arg("apigateway")
        .arg("create-rest-api")
        .args(&["--name", api_name])
        .args(&["--region", "us-east-1"])
        .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
        .output();

    let output = cmd.await.expect("failed to create rest api");

    let api: Value = serde_json::from_str(str::from_utf8(&output.stdout).unwrap())
        .expect("failed to parse the output");

    api["id"].to_string()
}

async fn parent_id(api_id: &str) -> String {
    let cmd = Command::new("aws")
        .arg("apigateway")
        .arg("get-resources")
        .args(&["--rest-api-id", api_id])
        .args(&["--region", "us-east-1"])
        .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
        .output();

    let output = cmd.await.expect("failed to get resources");

    let resources: Value = serde_json::from_str(str::from_utf8(&output.stdout).unwrap())
        .expect("failed to parse the output");

    resources["items"][0]["id"].to_string()
}

async fn create_resource(path_part: &str, parent_id: &str, api_id: &str) -> String {
    let cmd = Command::new("aws")
        .arg("apigateway")
        .arg("create-resource")
        .args(&["--path-part", path_part])
        .args(&["--parent-id", parent_id])
        .args(&["--rest-api-id", api_id])
        .args(&["--region", "us-east-1"])
        .args(&["--endpoint-url", LOCALSTACK_APIGATEWAY_ENDPOINT])
        .output();

    let output = cmd.await.expect("failed to create resource");

    let resource: Value = serde_json::from_str(str::from_utf8(&output.stdout).unwrap())
        .expect("failed to parse the output");

    resource["id"].to_string()
}
