use crate::authors;
use crate::emoji;
use console::style;
use failure;
use heck::{KebabCase, SnakeCase};
use liquid;
use quicli::prelude::*;
use std::fs;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub struct Name {
    pub user_input: String,
}

impl Name {
    pub fn new(name: &str) -> Name {
        Name {
            user_input: name.to_string(),
        }
    }

    pub fn kebab_case(&self) -> String {
        self.user_input.to_kebab_case()
    }

    pub fn snake_case(&self) -> String {
        self.user_input.to_snake_case()
    }
}

/// Taken from:
/// https://github.com/ashleygwilliams/cargo-generate/blob/5a2b7f988c448ccbda4b2d1c5c619125ccefcfaf/src/template.rs#L99
pub fn substitute(name: &Name) -> Result<liquid::value::Object, failure::Error> {
    let project_name = name.kebab_case();

    let mut template = liquid::value::Object::new();
    template.insert(
        "project-name".into(),
        liquid::value::Value::scalar(project_name),
    );
    template.insert(
        "crate_name".into(),
        liquid::value::Value::scalar(name.snake_case()),
    );
    template.insert(
        "authors".into(),
        liquid::value::Value::scalar(authors::get_authors()?),
    );
    Ok(template)
}

/// Adapted from:
/// https://github.com/ashleygwilliams/cargo-generate/blob/5a2b7f988c448ccbda4b2d1c5c619125ccefcfaf/src/template.rs#L121
pub fn walk_dir(
    project_dir: &PathBuf,
    template: liquid::value::Object,
) -> Result<(), failure::Error> {
    fn is_dir(entry: &DirEntry) -> bool {
        entry.file_type().is_dir()
    }

    fn is_git_metadata(entry: &DirEntry) -> bool {
        entry
            .path()
            .components()
            .any(|c| c == std::path::Component::Normal(".git".as_ref()))
    }

    let engine = liquid::ParserBuilder::new()
        .build()
        .expect("can't fail due to no partials support");

    for entry in WalkDir::new(project_dir) {
        let entry = entry?;
        if is_dir(&entry) || is_git_metadata(&entry) {
            continue;
        }

        let filename = entry.path();

        let new_contents = engine
            .clone()
            .parse_file(filename)?
            .render(&template)
            .with_context(|_e| {
                format!(
                    "{} {} `{}`",
                    emoji::ERROR,
                    style("Error replacing placeholders").bold().red(),
                    style(filename.display()).bold()
                )
            })?;
        fs::write(filename, new_contents).with_context(|_e| {
            format!(
                "{} {} `{}`",
                emoji::ERROR,
                style("Error writing").bold().red(),
                style(filename.display()).bold()
            )
        })?;
    }
    Ok(())
}
