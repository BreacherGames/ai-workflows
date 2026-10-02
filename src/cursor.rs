//! Install skills and the always-on engineering rule into Cursor.

use crate::pack::{self, Skill};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn install_skills(skills_root: &Path, skills: &[Skill]) -> Result<()> {
    fs::create_dir_all(skills_root)?;
    for skill in skills {
        let dir = skills_root.join(&skill.name);
        fs::create_dir_all(&dir)?;
        pack::write_text(&dir.join("SKILL.md"), &skill.markdown)?;
    }
    Ok(())
}

pub fn write_engineering_rule(path: &Path, body: &str) -> Result<()> {
    pack::write_text(path, body)
}

pub fn user_rule(rules_file: &Path) -> String {
    let rules = pack::posix(rules_file);
    format!(
        "---\n\
         description: Core engineering standards for AI assistants\n\
         alwaysApply: true\n\
         ---\n\n\
         Read and apply `{rules}` for the whole session,\n\
         including the daily workflow loop. Prefer installed Cursor skills for structured tasks.\n"
    )
}

pub fn project_rule() -> &'static str {
    "---\n\
     description: Core engineering standards for AI assistants\n\
     alwaysApply: true\n\
     ---\n\n\
     Read and apply `.ai-workflows/rules/engineering.md` for the whole session,\n\
     including the daily workflow loop. Prefer project Cursor skills for structured tasks.\n"
}
