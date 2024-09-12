use std::{
    path::Path,
    process::{Output, Stdio},
};

use anyhow::{anyhow, Result};
use command_group::{AsyncCommandGroup, AsyncGroupChild};
use tracing::info;

pub struct Command {
    command: tokio::process::Command,
}

pub struct CommandInstance {
    child: AsyncGroupChild,
}

impl Drop for CommandInstance {
    fn drop(&mut self) {
        drop(self.child.start_kill())
    }
}

impl From<AsyncGroupChild> for CommandInstance {
    fn from(child: AsyncGroupChild) -> Self {
        Self { child }
    }
}

impl Command {
    pub fn from_path(cmd_path: &Path) -> Result<Self> {
        let name = cmd_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("cmd");

        if !cmd_path.exists() {
            return Err(anyhow!(
                "{name} file not found at path: `{}`.
    This file is required to run the server.
            ",
                cmd_path.display()
            ));
        }

        info!("Running file `{}`", cmd_path.display());

        let mut command = tokio::process::Command::new(cmd_path);

        command.current_dir(&cmd_path.parent().unwrap_or(cmd_path));

        Ok(Self { command })
    }

    pub fn args(&mut self, args: &[&str]) {
        self.command.args(args);
    }

    pub fn run(&mut self) -> Result<CommandInstance> {
        info!("Output logs from setup script below");

        let child = self.command.group_spawn()?;

        Ok(child.into())
    }

    pub async fn run_and_capture(&mut self) -> Result<Output> {
        info!("Run process in background and capture its output");

        self.command.stdout(Stdio::piped());
        self.command.stderr(Stdio::piped());

        let output = self.command.group_output().await?;

        if output.status.success() {
            Ok(output)
        } else {
            info!("Stdout is:\n {}", String::from_utf8_lossy(&output.stdout));
            info!("Stderr is:\n {}", String::from_utf8_lossy(&output.stderr));

            Err(anyhow!(
                "Process failed with exit code: {}",
                output.status.code().unwrap_or(0)
            ))
        }
    }
}

impl CommandInstance {
    pub async fn kill(mut self) -> Result<()> {
        Ok(self.child.kill().await?)
    }
}
