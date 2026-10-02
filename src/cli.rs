use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "aiw",
    version,
    about = "Install team AI workflows into Cursor",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install or refresh workflows
    Install(ScopeArgs),
    /// Same as install
    Update(ScopeArgs),
    /// Remove installed workflows
    Remove(ScopeArgs),
    /// List workflows in the pack
    List,
    /// Check pack and install status
    Doctor,
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

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Scope {
    User,
    Project,
}
