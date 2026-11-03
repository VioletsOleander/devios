use std::process::Command;

use anyhow::Result;
use chrono::Local;

use super::get_code;
use crate::arg::BranchCommand;

pub struct Branch;

impl Branch {
    pub fn run(subcommand: BranchCommand) -> Result<String> {
        match subcommand {
            BranchCommand::Create { name } => {
                let time = Local::now().format("%Y%m%d-%H%M%S");
                let branch_name = format!("{}/{}", name, time);

                let _ = get_code(Command::new("git").args(["switch", "--create", &branch_name]));
                Ok(branch_name)
            }
        }
    }
}
