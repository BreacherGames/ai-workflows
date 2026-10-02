use crate::cli::{Scope, ScopeArgs};
use crate::cursor;
use crate::pack::{self, Skill};
use crate::paths;
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn list() -> Result<()> {
    let skills = pack::skills()?;
    println!("skills {}", skills.len());
    for skill in &skills {
        println!("{} - {}", skill.name, skill.description);
    }
    Ok(())
}

pub fn doctor() -> Result<()> {
    let skills = pack::skills()?;
    println!("aiw {}", env!("CARGO_PKG_VERSION"));
    println!("skills {}", skills.len());

    let user_pack = paths::user_pack_dir()?;
    println!(
        "user pack {} {}",
        status_label(user_pack.is_dir()),
        user_pack.display()
    );

    let skills_root = paths::cursor_user_skills()?;
    let installed = skills
        .iter()
        .filter(|skill| skills_root.join(&skill.name).join("SKILL.md").is_file())
        .count();
    println!("user skills {installed}/{}", skills.len());

    let rule = paths::cursor_user_rules()?.join("engineering.mdc");
    println!(
        "user rule {} {}",
        status_label(rule.is_file()),
        rule.display()
    );
    println!("ecosystem use aiw add owner/repo via skills.sh");
    Ok(())
}

pub fn install(args: ScopeArgs) -> Result<()> {
    let skills = pack::skills()?;
    match args.scope {
        Scope::User => install_user(&skills),
        Scope::Project => install_project(&resolve_project(args.project)?, &skills),
    }
}

pub fn remove(args: ScopeArgs) -> Result<()> {
    let skills = pack::skills()?;
    match args.scope {
        Scope::User => remove_user(&skills),
        Scope::Project => remove_project(&resolve_project(args.project)?, &skills),
    }
}

fn status_label(present: bool) -> &'static str {
    if present {
        "present"
    } else {
        "missing"
    }
}

fn resolve_project(project: Option<PathBuf>) -> Result<PathBuf> {
    let Some(project) = project else {
        bail!("--scope project requires --project <path>");
    };
    fs::canonicalize(&project).with_context(|| format!("resolve project {}", project.display()))
}

fn install_user(skills: &[Skill]) -> Result<()> {
    let pack_dir = paths::user_pack_dir()?;
    pack::materialize_vendor(&pack_dir)?;

    let skills_root = paths::cursor_user_skills()?;
    let rule_path = paths::cursor_user_rules()?.join("engineering.mdc");
    let rules_file = pack_dir.join("rules").join("engineering.md");

    cursor::install_skills(&skills_root, skills)?;
    cursor::write_engineering_rule(&rule_path, &cursor::user_rule(&rules_file))?;

    println!("pack {}", pack_dir.display());
    println!("skills {}", skills_root.display());
    println!("rule {}", rule_path.display());
    println!("done. restart Cursor to load skills and rules.");
    Ok(())
}

fn install_project(project: &Path, skills: &[Skill]) -> Result<()> {
    let vendor = paths::project_vendor(project);
    pack::materialize_vendor(&vendor)?;
    write_vendor_readme(&vendor, skills)?;

    let skills_root = paths::project_cursor_skills(project);
    let rule_path = paths::project_cursor_rules(project).join("engineering.mdc");
    cursor::install_skills(&skills_root, skills)?;
    cursor::write_engineering_rule(&rule_path, cursor::project_rule())?;

    println!("vendored {}", vendor.display());
    println!("skills .cursor/skills/");
    println!("rule .cursor/rules/engineering.mdc");
    println!("done.");
    Ok(())
}

fn write_vendor_readme(vendor: &Path, skills: &[Skill]) -> Result<()> {
    let names: Vec<_> = skills.iter().map(|skill| skill.name.as_str()).collect();
    let installed_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let body = format!(
        "# ai-workflows\n\n\
         Vendored by aiw {}.\n\
         Installed: {installed_at}\n\
         Skills: {}\n\n\
         Refresh with:\n\
         aiw install --scope project --project <path>\n\n\
         Or install skills only with:\n\
         npx skills add BreacherGames/ai-workflows\n\n\
         See LICENSE in this folder.\n",
        env!("CARGO_PKG_VERSION"),
        names.join(", "),
    );
    pack::write_text(&vendor.join("README.md"), &body)
}

fn remove_user(skills: &[Skill]) -> Result<()> {
    remove_skills(&paths::cursor_user_skills()?, skills)?;
    remove_file_if_exists(&paths::cursor_user_rules()?.join("engineering.mdc"))?;
    remove_dir_if_exists(&paths::user_pack_dir()?)?;

    let data_dir = paths::user_data_dir()?;
    if data_dir.is_dir() && fs::read_dir(&data_dir)?.next().is_none() {
        fs::remove_dir(&data_dir)?;
    }
    println!("done.");
    Ok(())
}

fn remove_project(project: &Path, skills: &[Skill]) -> Result<()> {
    remove_skills(&paths::project_cursor_skills(project), skills)?;
    remove_file_if_exists(&paths::project_cursor_rules(project).join("engineering.mdc"))?;
    remove_dir_if_exists(&paths::project_vendor(project))?;
    println!("done.");
    Ok(())
}

fn remove_skills(skills_root: &Path, skills: &[Skill]) -> Result<()> {
    for skill in skills {
        let dir = skills_root.join(&skill.name);
        if dir.is_dir() {
            fs::remove_dir_all(&dir)?;
            println!("removed skill {}", skill.name);
        }
    }
    Ok(())
}

fn remove_file_if_exists(path: &Path) -> Result<()> {
    if path.is_file() {
        fs::remove_file(path)?;
        println!("removed {}", path.display());
    }
    Ok(())
}

fn remove_dir_if_exists(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
        println!("removed {}", path.display());
    }
    Ok(())
}
