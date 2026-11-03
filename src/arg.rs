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
    ///
    /// The branch name will be <NAME> appended with a unique timestamp.
    Create {
        /// Name of the branch to create.
        name: String,
    },
}

#[derive(Subcommand)]
pub enum PRCommand {
    /// Create a new pull request on HEAD.
    ///
    /// The title and description of the pull request will be the commit message subject and body of
    /// HEAD.
    Create,
    /// Sqush merge the pull request on current branch.
    ///
    /// The merge commit message will have a footnote denoting the pull request URL.
    Merge,
}

#[derive(Subcommand)]
pub enum CommitCommand {
    /// Create a new commit on the main branch via a pr based process.
    ///
    /// The process has the following steps:
    ///
    /// 1. Create a new branch whose name is prefixed with <branch_name> and postfixed with a unique timestamp.
    /// 2. Switch to that branch and initiate git commit to commit the staged area.
    /// 3. After commit, create a new pull request with that commit.
    /// 4. Check pull request CI status and squash that pull request if CI is ok.
    /// 5. Switch to the main branch, pull the newly merged commit.
    /// 6. Delete the previouly created branch.
    Create {
        // Name of the branch to create for the pr process.
        branch_name: String,
    },
}
