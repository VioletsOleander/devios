use anyhow::Result;
use clap::Parser;

mod arg;
mod command;

use arg::{Args, BranchCommand, Command, CommitCommand, PRCommand};
use command::{Branch, PullRequest};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Branch(subcommand) => {
            Branch::run(subcommand)?;
        }
        Command::PR(subcommand) => {
            PullRequest::run(subcommand)?;
        }
        Command::Commit(_) => (),
    };

    Ok(())
}
