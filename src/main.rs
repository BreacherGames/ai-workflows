mod cli;
mod cursor;
mod ecosystem;
mod install;
mod pack;
mod paths;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Install(args) | Command::Update(args) => install::install(args),
        Command::Remove(args) => install::remove(args),
        Command::List => install::list(),
        Command::Doctor => install::doctor(),
        Command::Add(args) => ecosystem::add(args),
        Command::Find(args) => ecosystem::find(args),
    }
}
