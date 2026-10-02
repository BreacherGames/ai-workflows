use anyhow::{bail, Context, Result};
use include_dir::{include_dir, Dir};
use std::path::Path;

static SKILLS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills");
static RULES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/rules");
const LICENSE: &str = include_str!("../LICENSE");

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub markdown: String,
}

struct Frontmatter {
    name: String,
    description: String,
}

pub fn skills() -> Result<Vec<Skill>> {
    let mut skills = Vec::new();
    for dir in SKILLS.dirs() {
        let name = dir
            .path()
            .file_name()
            .and_then(|value| value.to_str())
            .context("invalid skill directory name")?
            .to_string();
        let relative = format!("{name}/SKILL.md");
        let file = SKILLS
            .get_file(&relative)
            .with_context(|| format!("missing skills/{relative}"))?;
        let markdown = file
            .contents_utf8()
            .with_context(|| format!("skills/{relative} is not UTF-8"))?
            .to_string();
        let frontmatter = parse_frontmatter(&markdown)
            .with_context(|| format!("parse frontmatter in skills/{relative}"))?;
        if frontmatter.name != name {
            bail!(
                "skill folder {name} does not match frontmatter name {}",
                frontmatter.name
            );
        }
        skills.push(Skill {
            name,
            description: collapse_whitespace(&frontmatter.description),
            markdown,
        });
    }
    skills.sort_by(|left, right| left.name.cmp(&right.name));
    if skills.is_empty() {
        bail!("no skills found in embedded skills/");
    }
    Ok(skills)
}

pub fn engineering_rule() -> Result<&'static str> {
    RULES
        .get_file("engineering.md")
        .context("missing rules/engineering.md")?
        .contents_utf8()
        .context("rules/engineering.md is not UTF-8")
}

pub fn materialize_vendor(dest: &Path) -> Result<()> {
    if dest.exists() {
        std::fs::remove_dir_all(dest).with_context(|| format!("clean {}", dest.display()))?;
    }
    std::fs::create_dir_all(dest.join("skills"))?;
    std::fs::create_dir_all(dest.join("rules"))?;

    for skill in skills()? {
        let skill_dir = dest.join("skills").join(&skill.name);
        std::fs::create_dir_all(&skill_dir)?;
        write_text(&skill_dir.join("SKILL.md"), &skill.markdown)?;
    }

    write_text(
        &dest.join("rules").join("engineering.md"),
        engineering_rule()?,
    )?;
    write_text(&dest.join("LICENSE"), LICENSE)?;
    Ok(())
}

pub fn write_text(path: &Path, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let body = if text.ends_with('\n') {
        text.to_string()
    } else {
        format!("{text}\n")
    };
    std::fs::write(path, body).with_context(|| format!("write {}", path.display()))
}

pub fn posix(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn parse_frontmatter(markdown: &str) -> Result<Frontmatter> {
    let rest = markdown
        .strip_prefix("---\n")
        .or_else(|| markdown.strip_prefix("---\r\n"))
        .context("SKILL.md must start with YAML frontmatter")?;
    let end = rest
        .find("\n---\n")
        .or_else(|| rest.find("\n---\r\n"))
        .context("SKILL.md frontmatter is not closed")?;
    parse_frontmatter_yaml(&rest[..end])
}

fn parse_frontmatter_yaml(yaml: &str) -> Result<Frontmatter> {
    let mut name = None;
    let mut description_lines = Vec::new();
    let mut in_description = false;

    for line in yaml.lines() {
        if let Some(value) = line.strip_prefix("name:") {
            name = Some(value.trim().trim_matches('"').to_string());
            in_description = false;
            continue;
        }
        if let Some(rest) = line.strip_prefix("description:") {
            in_description = true;
            let rest = rest.trim();
            if rest == ">-" || rest == "|" || rest.is_empty() {
                continue;
            }
            description_lines.push(rest.trim_matches('"').to_string());
            in_description = false;
            continue;
        }
        if line.starts_with("disable-model-invocation:") {
            in_description = false;
            continue;
        }
        if in_description {
            description_lines.push(line.trim().to_string());
        }
    }

    Ok(Frontmatter {
        name: name.context("frontmatter missing name")?,
        description: description_lines.join(" "),
    })
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skills_are_valid() {
        let loaded = skills().expect("skills");
        assert_eq!(loaded.len(), 11);
        assert!(loaded.iter().any(|skill| skill.name == "implement"));
        assert!(loaded.iter().any(|skill| skill.name == "commit"));
        for skill in &loaded {
            assert!(!skill.description.is_empty(), "{}", skill.name);
            assert!(skill.markdown.contains("disable-model-invocation"), "{}", skill.name);
        }
    }

    #[test]
    fn engineering_rule_exists() {
        assert!(engineering_rule().unwrap().contains("Daily loop"));
    }
}
