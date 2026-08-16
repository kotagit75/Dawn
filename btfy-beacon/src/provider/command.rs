use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    time::timeout,
};

use crate::{location::BeaconLocation, provider::BeaconProvider};

#[derive(Debug, Deserialize)]
struct BeaconResponse {
    temperature: i32,
}

pub struct CommandBeaconProvider {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl CommandBeaconProvider {
    pub fn spawn(command: &[String]) -> Result<Self> {
        if command.is_empty() {
            return Err(anyhow!("beacon command is not configured"));
        }

        let mut child = Command::new(&command[0]);
        child
            .args(&command[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());

        let mut child = child
            .spawn()
            .map_err(|err| anyhow!("failed to start beacon process: {}", err))?;

        let Some(stdin) = child.stdin.take() else {
            return Err(anyhow!("failed to open beacon process stdin"));
        };
        let Some(stdout) = child.stdout.take() else {
            return Err(anyhow!("failed to open beacon process stdout"));
        };

        Ok(Self {
            _child: child,
            stdin,
            stdout: BufReader::new(stdout),
        })
    }
}

use std::{future::Future, pin::Pin};

impl BeaconProvider for CommandBeaconProvider {
    fn fetch_temperature<'a>(
        &'a mut self,
        location: &'a BeaconLocation,
        timestamp: i64,
    ) -> Pin<Box<dyn Future<Output = Option<i32>> + Send + 'a>> {
        Box::pin(async move {
            let timeout_duration = std::time::Duration::from_secs(5);

            timeout(timeout_duration, async {
                self.stdin
                    .write_all(
                        format!(
                            "{} {} {} {}\n",
                            location.lat(),
                            location.lon(),
                            location.icao_code(),
                            timestamp
                        )
                        .as_bytes(),
                    )
                    .await
                    .ok()?;
                self.stdin.flush().await.ok()?;

                let mut line = String::new();
                let read = self.stdout.read_line(&mut line).await.ok()?;
                if read == 0 {
                    return None;
                }

                serde_json::from_str::<BeaconResponse>(line.trim())
                    .ok()
                    .map(|r| r.temperature)
            })
            .await
            .ok()
            .flatten()
        })
    }
}
