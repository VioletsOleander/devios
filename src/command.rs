use std::process::Command;

use anyhow::{Result, anyhow};

mod branch;
mod commit;
mod pr;

pub use branch::Branch;
pub use pr::PullRequest;

fn wait(command: &mut Command) -> Result<()> {
    let status = command.status()?;

    if !status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "command: {:?} failed with exit code {:?}",
            command,
            status.code(),
        ))
    }
}

fn wait_output(command: &mut Command) -> Result<String> {
    let output = command.output()?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    } else {
        Err(anyhow!(
            "command: {:?} failed with exit code {:?} and stderr {}",
            command,
            output.status.code(),
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}
