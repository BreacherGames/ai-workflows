use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub fn user_data_dir() -> Result<PathBuf> {
    let base = dirs::data_local_dir().context("could not resolve local data directory")?;
    Ok(base.join("aiw"))
}

pub fn user_pack_dir() -> Result<PathBuf> {
    Ok(user_data_dir()?.join("pack"))
}

pub fn cursor_user_skills() -> Result<PathBuf> {
    Ok(cursor_user_root()?.join("skills"))
}

pub fn cursor_user_rules() -> Result<PathBuf> {
    Ok(cursor_user_root()?.join("rules"))
}

pub fn project_vendor(project: &Path) -> PathBuf {
    project.join(".ai-workflows")
}

pub fn project_cursor_skills(project: &Path) -> PathBuf {
    project_cursor_root(project).join("skills")
}

pub fn project_cursor_rules(project: &Path) -> PathBuf {
    project_cursor_root(project).join("rules")
}

fn cursor_user_root() -> Result<PathBuf> {
    Ok(dirs::home_dir()
        .context("could not resolve home directory")?
        .join(".cursor"))
}

fn project_cursor_root(project: &Path) -> PathBuf {
    project.join(".cursor")
}
