use clap::Args;
use std::path::PathBuf;

use crate::options::GlobalOptions;

#[derive(Debug, Args)]
pub struct ReadArgs {
    /// An example option
    #[clap(long, short = 'o')]
    example_opt: bool,

    /// The path to read from
    path: PathBuf,
}

pub fn run_read(global_opts: &GlobalOptions, args: &ReadArgs) {
    println!("Running read");
    dbg!(global_opts, args);
}
