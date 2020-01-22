use cargo_ease::create;
use cargo_ease::Args;
use cargo_ease::Cli;
use quicli::prelude::*;
use structopt::StructOpt;

fn main() -> CliResult {
    let args: Args = match Cli::from_args() {
        Cli::Ease(args) => args,
    };

    create(args)?;
    Ok(())
}
