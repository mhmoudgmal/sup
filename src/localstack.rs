pub mod aws;

use std::str;

use log::*;
use colored::*;
use tokio::process::Command;

const CONTAINER_NAME: &str = "lsup_localstack";

pub async fn running_version() -> Result<String, Box<dyn std::error::Error>> {
    let running_version_command = Command::new("docker")
        .arg("inspect")
        .arg(CONTAINER_NAME)
        .args(&["-f", "{{.Config.Image}}"])
        .output();

    let output = running_version_command.await?;

    let image = str::from_utf8(&output.stdout).unwrap();

    let tokens: Vec<&str> = image.trim().split(":").collect();
    let version = tokens[1].to_string();

    Ok(version)
}

pub async fn is_running() -> Result<bool, Box<dyn std::error::Error>> {
    let is_running_command = Command::new("docker")
        .arg("inspect")
        .arg(CONTAINER_NAME)
        .args(&["-f", "{{.State.Running}}"])
        .output();

    let output = is_running_command.await?;

    if output.stdout == b"true\n" {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub async fn start(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let localstack_version = ensure_version(version);

    let localstack_process = Command::new("docker")
        .arg("run")
        .arg("-d")
        .args(&["-p", "4567-4583:4567-4583"])
        .args(&["-v", "/var/run/docker.sock:/var/run/docker.sock"])
        .args(&["-v", "/tmp/localstack:/tmp/localstack"])
        .args(&["-e", "SERVICES=${SERVICES- }"])
        .args(&["-e", "DEBUG=${DEBUG- }"])
        .args(&["-e", "DATA_DIR=${DATA_DIR- }"])
        .args(&["-e", "PORT_WEB_UI=${PORT_WEB_UI- }"])
        .args(&["-e", "LAMBDA_EXECUTOR=${LAMBDA_EXECUTOR- }"])
        .args(&["-e", "DOCKER_HOST=unix:///var/run/docker.sock"])
        .args(&[
            "-e",
            "KINESIS_ERROR_PROBABILITY=${KINESIS_ERROR_PROBABILITY- }",
        ])
        .args(&["--name", CONTAINER_NAME])
        .arg(format!("localstack/localstack:{}", localstack_version))
        .spawn()
        .expect("failed to run the container");

    info!("starting localstack version {}", localstack_version.yellow());
    localstack_process.await?;

    Ok(())
}

pub async fn stop() -> Result<(), Box<dyn std::error::Error>> {
    let localstack_process = Command::new("docker")
        .arg("stop")
        .arg(CONTAINER_NAME)
        .spawn()
        .expect("failed to stop the container");

    localstack_process.await?;

    Ok(())
}

pub async fn remove() -> Result<(), Box<dyn std::error::Error>> {
    let localstack_process = Command::new("docker")
        .arg("rm")
        .arg(CONTAINER_NAME)
        .spawn()
        .expect("failed to remove the container");

    localstack_process.await?;

    Ok(())
}

fn ensure_version(version: &str) -> &str {
    if version.is_empty() {
        return "latest";
    }

    return version;
}
