use clap::Parser;
use colored::Colorize;

use cmd::Command;
use options::GlobalOptions;

mod cmd;
mod metadata;
mod options;

#[derive(Debug, Parser)]
#[clap(name = "cargo genezio", version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOptions,

    #[clap(subcommand)]
    command: Command,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::parse();

    if let Err(e) = app.command.run(&app.global_opts) {
        eprintln!("{}", e.to_string().red());

        std::process::exit(1);
    }

    Ok(())
}
