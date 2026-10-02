use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "aiw",
    version,
    about = "Team AI workflows for Cursor, with skills.sh for the open ecosystem",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install the Breacher pack into Cursor
    Install(ScopeArgs),
    /// Same as install
    Update(ScopeArgs),
    /// Remove the Breacher pack from Cursor
    Remove(ScopeArgs),
    /// List skills in this pack
    List,
    /// Check pack and install status
    Doctor,
    /// Add a skill from the open ecosystem via skills.sh
    Add(AddArgs),
    /// Search skills.sh
    Find(FindArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct ScopeArgs {
    /// user or project
    #[arg(long, value_enum, default_value_t = Scope::User)]
    pub scope: Scope,

    /// Project root. Required with --scope project
    #[arg(long)]
    pub project: Option<PathBuf>,
}

#[derive(Debug, Clone, Parser)]
pub struct AddArgs {
    /// GitHub source, for example owner/repo
    pub source: String,

    /// Install one skill from the repo
    #[arg(long, short = 's')]
    pub skill: Option<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct FindArgs {
    /// Search query
    pub query: Option<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Scope {
    User,
    Project,
}
