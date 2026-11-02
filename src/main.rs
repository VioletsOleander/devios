use anyhow::Result;
use clap::Parser;

mod arg;
mod command;

use arg::{Args, BranchCommand, Command, CommitCommand, PRCommand};
use command::Branch;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Branch(subcommand) => {
            Branch::run(subcommand)?;
        }
        Command::PR(_) => (),
        Command::Commit(_) => (),
    };

    Ok(())
}
