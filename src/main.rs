use cargo_ease::create;
use cargo_ease::Cli;
use quicli::prelude::*;
use structopt::StructOpt;

fn main() -> CliResult {
    let Cli::Ease(args) = Cli::from_args();
    create(args)?;
    Ok(())
}
