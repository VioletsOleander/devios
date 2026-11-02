use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manipulate branches.
    #[command(subcommand)]
    Branch(BranchCommand),
    /// Manipulate pull requests.
    #[command(subcommand)]
    PR(PRCommand),
    /// Manipulate commits.
    #[command(subcommand)]
    Commit(CommitCommand),
}

#[derive(Subcommand)]
pub enum BranchCommand {
    /// Create a new branch with a timestamp.
    Create {
        /// Name of the branch to create.
        name: String,
    },
}

#[derive(Subcommand)]
pub enum PRCommand {
    /// Create a new pull request on HEAD.
    Create,
    /// Sqush merge the pull request on current branch.
    Merge,
}

#[derive(Subcommand)]
pub enum CommitCommand {
    /// Create a new commit on the main branch via a pr process.
    Create,
}
