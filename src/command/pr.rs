use std::process::Command;

use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;

use super::{get_code, get_output};
use crate::arg::PRCommand;

pub struct PullRequest;

#[derive()]
struct PullRequestInfo {
    title: String,
    body: String,
    url: String,
    status_check_rollup: String,
}

impl PullRequest {
    pub fn run(subcommand: PRCommand) -> Result<()> {
        match subcommand {
            PRCommand::Create => {
                let subject = get_output(Command::new("git").args(["log", "-1", "--format=%s"]))?;
                let body = get_output(Command::new("git").args(["log", "-1", "--format=%b"]))?;

                let _ = get_code(
                    Command::new("gh").args(["pr", "create", "--title", &subject, "--body", &body]),
                );

                Ok(())
            }
            PRCommand::Merge => todo!(),
            // PRCommand::Merge => {
            //     let info = get_output(Command::new("gh").args([
            //         "pr",
            //         "view",
            //         "--json",
            //         "title,body,url,statusCheckRollup",
            //     ]))?;
            // }
        }
    }
}
