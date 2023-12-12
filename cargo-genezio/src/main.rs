use clap::Parser;
use cmd::Command;
use options::GlobalOptions;

mod cmd;
mod options;

#[derive(Debug, Parser)]
#[clap(name = "cargo-genezio", version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOptions,

    #[clap(subcommand)]
    command: Command,
}

fn main() {
    let app = App::parse();
    app.command.run(&app.global_opts);
}
