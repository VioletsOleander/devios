use std::process::Command;

use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;

use super::{wait, wait_output};
use crate::arg::PRCommand;

pub struct PullRequest;

#[derive(Deserialize)]
struct PullRequestInfo {
    title: String,
    body: String,
    url: String,
    #[serde(rename = "statusCheckRollup")]
    status_check_rollup: Vec<Value>,
}

impl PullRequest {
    pub fn run(subcommand: PRCommand) -> Result<()> {
        match subcommand {
            PRCommand::Create => {
                let subject = wait_output(Command::new("git").args(["log", "-1", "--format=%s"]))?;
                let body = wait_output(Command::new("git").args(["log", "-1", "--format=%b"]))?;

                wait(
                    Command::new("gh").args(["pr", "create", "--title", &subject, "--body", &body]),
                )?;

                Ok(())
            }
            PRCommand::Merge => {
                let output = wait_output(Command::new("gh").args([
                    "pr",
                    "view",
                    "--json",
                    "title,body,url,statusCheckRollup",
                ]))?;

                let info: PullRequestInfo = serde_json::from_str(&output)?;

                if !info.status_check_rollup.is_empty() {
                    wait(Command::new("gh").args(["pr", "checks", "--watch", "--fail-fast"]))?;
                }

                let subject = info.title;
                let trimmed_body = info.body.trim_end();
                let body = if trimmed_body.is_empty() {
                    format!("PR: {}", info.url)
                } else {
                    format!("{}\n\nPR: {}", trimmed_body, info.url)
                };

                wait(Command::new("gh").args([
                    "pr",
                    "merge",
                    "--squash",
                    "--subject",
                    &subject,
                    "--body",
                    &body,
                ]))?;

                Ok(())
            }
        }
    }
}
