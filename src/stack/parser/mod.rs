use std::collections::HashMap;
use std::fs;

use colored::*;
use log::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AWSService {
    Lambda {
        runtime: String,
        handler: String,
        #[serde(default)]
        env_file: String,
        #[serde(default)]
        env_vars: HashMap<String, String>,
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
        #[serde(default)]
        shard_count: u32,
    },
    S3 {
        bucket: String,
        #[serde(default)]
        files: HashMap<String, String>,
    },
    SNS {},
    SQS {},
}

#[derive(Deserialize, Debug)]
pub struct Dep {
    #[serde(default)]
    pub services: Vec<String>,
    pub location: String,
    #[serde(default)]
    pub stackfile: String,
}

#[derive(Deserialize, Debug)]
pub struct Service {
    #[serde(flatten)]
    pub variant: AWSService,
    #[serde(default)]
    pub deps: HashMap<String, Dep>,
}

#[derive(Deserialize, Debug)]
pub struct LocalstackConfig {
    pub version: String,
    pub services: Vec<String>,
    pub docker_host: String,
    #[serde(default)]
    pub lambda_executer: String,
    #[serde(default)]
    pub data_dir: String,
    #[serde(default)]
    pub port_web_ui: String,
    #[serde(default)]
    pub debug: String,
    #[serde(default)]
    pub kinesis_error_probability: u32,
}

#[derive(Deserialize, Debug)]
pub struct Stack {
    pub localstack_config: LocalstackConfig,
    pub services: HashMap<String, Service>,
}

pub fn parse(stackfile: &str, format: &str) -> Option<Stack> {
    let data = fs::read_to_string(&stackfile).expect("failed reading stackfile");

    match format.as_ref() {
        "json" => serde_json::from_str(&data).unwrap(),
        "yaml" => serde_yaml::from_str(&data).unwrap(),
        _ => {
            error!(
                "don't know how to parse '{}', not supported stackfile format",
                stackfile.yellow()
            );
            return None;
        }
    }
}
