use std::error::Error;

use colored::*;
use log::*;
use tokio::process::Command;

use crate::stack::parser::*;

const LOCALSTACK_KINESIS_ENDPOINT: &str = "http://localhost:4568";

pub async fn wait_for_it() -> Result<(), Box<dyn Error>> {
    let mut ready: bool = false;

    while !ready {
        warn!("waiting for localstack's kinesis to be ready..");

        let cmd = Command::new("aws")
            .arg("kinesis")
            .arg("list-streams")
            .args(&["--endpoint-url", LOCALSTACK_KINESIS_ENDPOINT])
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
        AWSService::Kinesis {
            stream_name,
            shard_count,
        } => {
            info!("\ndeploying kinesis service '{}'\n", name.yellow());

            Command::new("aws")
                .arg("kinesis")
                .arg("create-stream")
                .args(&["--stream-name", &stream_name])
                .args(&[
                    "--shard-count",
                    &ensure_shard_count(shard_count).to_string(),
                ])
                .args(&["--endpoint-url", LOCALSTACK_KINESIS_ENDPOINT])
                .status()
                .await
                .expect("failed to create stream");
        }

        _ => (),
    }
}

fn ensure_shard_count(shard_count: u32) -> u32 {
    if shard_count < 1 {
        return 1;
    } else {
        return shard_count;
    }
}
