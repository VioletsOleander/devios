use anyhow::Result;
use clap::Parser;

mod arg;
mod command;

use arg::{Args, Command};
use command::{Branch, Commit, PullRequest};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Branch(subcommand) => {
            Branch::run(subcommand)?;
        }
        Command::PR(subcommand) => {
            PullRequest::run(subcommand)?;
        }
        Command::Commit(subcommand) => {
            Commit::run(subcommand)?;
        }
    };

    Ok(())
}
