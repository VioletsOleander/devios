use std::process::Command;

use anyhow::{Result, anyhow};
use chrono::Local;

use crate::arg::BranchCommand;

pub struct Branch;

impl Branch {
    pub fn run(subcommand: BranchCommand) -> Result<String> {
        match subcommand {
            BranchCommand::Create { name } => {
                let time = Local::now().format("%Y%m%d-%H%M%S");
                let branch_name = format!("{}/{}", name, time);

                let status = Command::new("git")
                    .args(["switch", "--create", &branch_name])
                    .status()?;

                if status.success() {
                    Ok(branch_name)
                } else {
                    Err(anyhow!(
                        "failed to run command git switch --create {}, received exit code {:?}",
                        branch_name,
                        status.code()
                    ))
                }
            }
        }
    }
}
