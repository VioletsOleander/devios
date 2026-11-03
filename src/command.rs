use std::process::Command;

use anyhow::{Result, anyhow};

mod branch;
mod commit;
mod pr;

pub use branch::Branch;

fn get_output(command: &mut Command) -> Result<String> {
    let output = command.output()?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    } else {
        Err(anyhow!(
            "failed to run command: {:?}, with exit code {:?} and stderr {}",
            command,
            output.status.code(),
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}

fn get_code(command: &mut Command) -> Result<Option<i32>> {
    let status = command.status()?;

    if status.success() {
        Ok(status.code())
    } else {
        Err(anyhow!(
            "failed to run command: {:?}, with exit code {:?}",
            command,
            status.code(),
        ))
    }
}
