pub mod aws;

use std::process::ExitStatus;
use std::str;

use tokio::process::Command;

use crate::localstack;
use crate::stack::parser::LocalstackConfig;

const CONTAINER_NAME: &str = "sup_localstack";

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

pub async fn start(config: &LocalstackConfig) {
    let version = ensure_version(&config.version);
    let services = config.services.join(",");

    Command::new("docker")
        .arg("run")
        .arg("-d")
        .args(&["-p", "4567-4583:4567-4583"])
        .args(&["-v", "/var/run/docker.sock:/var/run/docker.sock"])
        .args(&["-v", "/tmp/localstack:/tmp/localstack"])
        .args(&["-e", &format!("SERVICES={}", services)])
        .args(&["-e", &format!("DEBUG={}", config.debug)])
        .args(&["-e", &format!("DATA_DIR={}", config.data_dir)])
        .args(&["-e", &format!("PORT_WEB_UI={}", config.port_web_ui)])
        .args(&["-e", &format!("LAMBDA_EXECUTOR={}", config.lambda_executer)])
        .args(&["-e", &format!("DOCKER_HOST={}", config.docker_host)])
        .args(&[
            "-e",
            &format!(
                "KINESIS_ERROR_PROBABILITY={}",
                config.kinesis_error_probability
            ),
        ])
        .args(&["--name", CONTAINER_NAME])
        .arg(format!("localstack/localstack:{}", version))
        .status()
        .await
        .expect("failed to start localstack");

    localstack::aws::wait_for_it(&config.services).await;
}

pub async fn stop() -> ExitStatus {
    Command::new("docker")
        .arg("stop")
        .arg(CONTAINER_NAME)
        .status()
        .await
        .expect("failed to stop localstack")
}

pub async fn remove() -> ExitStatus {
    Command::new("docker")
        .arg("rm")
        .arg(CONTAINER_NAME)
        .status()
        .await
        .expect("failed to remove the container")
}

fn ensure_version(version: &str) -> &str {
    if version.is_empty() {
        return "latest";
    }

    return version;
}
