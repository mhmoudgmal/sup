use std::collections::HashMap;
use std::fs;
use std::process::exit;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Service {
    Lambda {
        runtime: String,
        handler: String,
        env_file: String,
        function: String,
        files: Vec<String>,
        function_path: String,
    },
    Apigateway {
        api_name: String,
        path_part: String,
        http_method: String,
        authorization_type: String,
        integration_type: String,
        integration_http_method: String,
        uri: String,
    },
    Dynamo {
        table_name: String,
        schema_file: String,
    },
    Kinesis {
        stream_name: String,
    },
    S3 {
        bucket: String,
        files: HashMap<String, String>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Stack {
    localstack_version: String,
    services: HashMap<String, Service>,
}

pub fn parse(stackfile: &str, format: &str) -> Stack {
    let data = fs::read_to_string(&stackfile).expect("failed reading stackfile");

    match format.as_ref() {
        "json" => serde_json::from_str(&data).unwrap(),
        "yaml" => serde_yaml::from_str(&data).unwrap(),
        _ => {
            println!(
                "don't know how to parse '{}', not supported stackfile format",
                stackfile
            );
            exit(1)
        }
    }
}
