use std::process::Command;

use anyhow::Result;

use super::wait;
use super::{Branch, PullRequest};
use crate::arg::{BranchCommand, CommitCommand, PRCommand};

pub struct Commit;

impl Commit {
    pub fn run(subcommand: CommitCommand) -> Result<()> {
        match subcommand {
            CommitCommand::Create { branch_name } => {
                let branch_name = Branch::run(BranchCommand::Create { name: branch_name })?;
                wait(Command::new("git").arg("commit"))?;
                wait(Command::new("git").arg("push"))?;

                PullRequest::run(PRCommand::Create)?;
                PullRequest::run(PRCommand::Merge)?;

                wait(Command::new("git").args(["switch", "main"]))?;
                wait(Command::new("git").args(["pull", "--prune"]))?;
                wait(Command::new("git").args(["branch", "-D", &branch_name]))?;

                Ok(())
            }
        }
    }
}
