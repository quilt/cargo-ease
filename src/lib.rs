mod authors;
mod git;
mod template;
mod emoji;

use crate::git::GitConfig;
use crate::template::Name;
use cargo;
use console::style;
use failure;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;
use dialoguer::Input;
use quicli::prelude::Error;

static EE_TEMPLATE: &str = "https://github.com/jrhea/ease-ee-template.git";

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Cli {
    #[structopt(name = "ease")]
    Ease(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "branch")]
    branch: Option<String>,
    #[structopt(long = "name", short = "n")]
    name: Option<String>,
}

pub fn prompt_for_name() -> Result<String, Error> {
    let valid_ident = regex::Regex::new(r"^([a-zA-Z][a-zA-Z0-9_-]+)$")?;
    let name = loop {
        let name = Input::new(&format!(
            "{} {}",
            emoji::QUESTION,
            style("Project Name").bold()
        ))
        .interact()?;
        if valid_ident.is_match(&name) {
            break name;
        } else {
            eprintln!(
                "{} {} \"{}\" {}",
                emoji::WARN,
                style("Sorry,").bold().red(),
                style(&name).bold().yellow(),
                style("is not a valid crate name").bold().red()
            );
        }
    };
    Ok(name)
}

pub fn create(args: Args) -> Result<(), failure::Error> {
    let name : &Name = &match &args.name {
        Some(ref n) => Name::new(n),
        None => Name::new(&prompt_for_name()?),  //Prompt user
    };
    let branch = args.branch.unwrap_or_else(|| "master".to_string());
    let config = GitConfig::new(EE_TEMPLATE.to_string(), branch.clone())?;
    if let Some(dir) = &create_project_dir(&name) {
        match git::create(dir, config) {
            Ok(_) => {
                git::remove_history(dir).unwrap_or(apply_template(name, dir, &branch)?)
            }
            Err(e) => failure::bail!(
                "{} {} {}",
                emoji::ERROR,
                style("Git Error:").bold().red(),
                style(e).bold().red(),
            ),
        };
    } else {
        failure::bail!(
            "{} {}",
            emoji::ERROR,
            style("Error: directory already exists!")
                .bold()
                .red(),
        );
    }
    Ok(())
}

/// Adapted from: 
/// https://github.com/ashleygwilliams/cargo-generate/blob/5a2b7f988c448ccbda4b2d1c5c619125ccefcfaf/src/lib.rs#L110
fn create_project_dir(name: &Name) -> Option<PathBuf> {
    let dir_name = name.kebab_case();
    let project_dir = env::current_dir()
        .unwrap_or_else(|_e| ".".into())
        .join(&dir_name);

    println!(
        "{} {} `{}`{}",
        emoji::PICKAXE,
        style("Creating project ").bold(),
        style(&dir_name).bold().yellow(),
        style("...").bold()
    );

    if project_dir.exists() {
        None
    } else {
        Some(project_dir)
    }
}

fn apply_template(
    name: &Name,
    dir: &PathBuf,
    branch: &str,
) -> Result<(), failure::Error> {
    let template = template::substitute(name)?;
    template::walk_dir(dir, template)?;
    git::init(dir, branch)?;
    let dir_string = dir.to_str().unwrap_or("");
    println!(
        "{} {} {} {}",
        emoji::UNICORN,
        style("Boom!").bold().green(),
        style("New project created").bold(),
        style(dir_string).underlined()
    );
    Ok(())
}
