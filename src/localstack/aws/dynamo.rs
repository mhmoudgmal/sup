use std::path::Path;

use log::*;
use colored::*;
use tokio::process::Command;

use crate::stack::parser::*;

const LOCALSTACK_DYNAMODB_ENDPOINT: &str = "http://localhost:4569";

pub async fn deploy((name, service): (String, AWSService)) {
    match service {
        AWSService::Dynamo { table_name, schema, recreate } => {
            if recreate {
                info!("deleting dynamodb table '{}'", name.yellow());
                delete_table(&table_name).await
            }

            info!("creating dynamodb table '{}'", name.yellow());

            let sch;
            // NOTE:
            // - when schema is specified as an object, it's expected to -
            // have proper schema with the expected fields in a json object
            //
            // - when schema is specified as a string, it's expected -
            // to refer to a json file contains the proper schema with the expected fields
            if schema.is_object() {
                sch = schema.to_string();
            } else if schema.is_string() {
                sch = match schema.as_str() {
                    Some(f) => if Path::new(f).exists() { String::from(f) } else {
                        warn!("schema file does not exist");
                        warn!("skipping '{}'", name.yellow());
                        return;
                    }
                    None => {
                        warn!("proper schema is required");
                        warn!("skipping '{}'", name.yellow());
                        return;
                    }
                }
            } else {
                warn!("proper schema is required, expected a json object or a json file");
                warn!("skipping '{}'", name.yellow());
                return;
            }

            Command::new("aws")
                .arg("dynamodb")
                .arg("create-table")
                .args(&["--table-name", &table_name])
                .args(&["--cli-input-json", &sch])
                .args(&["--endpoint-url", LOCALSTACK_DYNAMODB_ENDPOINT])
                .status()
                .await
                .expect("failed to create dynamodb table");
        },

        _ => ()
    }

    println!("\n")
}

async fn delete_table(table_name: &str) {
    Command::new("aws")
        .arg("dynamodb")
        .arg("delete-table")
        .args(&["--table-name", &table_name])
        .args(&["--endpoint-url", LOCALSTACK_DYNAMODB_ENDPOINT])
        .status()
        .await
        .expect("failed to delete dynamodb table");

    println!("\n")
}
