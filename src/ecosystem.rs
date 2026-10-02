//! Thin wrapper around the skills.sh CLI. No second registry.

use crate::cli::{AddArgs, FindArgs};
use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn add(args: AddArgs) -> Result<()> {
    validate_source(&args.source)?;

    let mut skills_args = vec!["add".to_string(), args.source.clone()];
    if let Some(skill) = &args.skill {
        skills_args.push("--skill".to_string());
        skills_args.push(skill.clone());
    }

    run_skills(&skills_args)
}

pub fn find(args: FindArgs) -> Result<()> {
    let mut skills_args = vec!["find".to_string()];
    if let Some(query) = args.query {
        skills_args.push(query);
    }
    run_skills(&skills_args)
}

fn validate_source(source: &str) -> Result<()> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        bail!("source is empty");
    }

    let looks_remote = trimmed.contains('/')
        || trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with('.');

    if !looks_remote {
        bail!(
            "expected a GitHub source like owner/repo.\n\
             For the Breacher pack use: aiw install\n\
             For one skill from a repo use: aiw add owner/repo --skill name"
        );
    }

    Ok(())
}

fn run_skills(args: &[String]) -> Result<()> {
    let display = format!("npx skills {}", args.join(" "));
    println!("running {display}");

    let status = npx_skills(args)
        .status()
        .context("failed to run npx skills. Install Node.js so npx is on PATH")?;

    if !status.success() {
        bail!("{display} failed with {status}");
    }

    Ok(())
}

fn npx_skills(args: &[String]) -> Command {
    let mut command = if cfg!(windows) {
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "npx", "--yes", "skills"]);
        cmd
    } else {
        let mut cmd = Command::new("npx");
        cmd.args(["--yes", "skills"]);
        cmd
    };
    command.args(args);
    command
}
