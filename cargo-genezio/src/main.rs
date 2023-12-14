use clap::Parser;
use colored::Colorize;

use cmd::Command;
use options::GlobalOptions;

mod cmd;
mod metadata;
mod options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Command::parse();

    if let Err(e) = app.run(&GlobalOptions {}) {
        eprintln!("{}", e.to_string().red());

        std::process::exit(1);
    }

    Ok(())
}
