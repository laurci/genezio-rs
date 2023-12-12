use clap::Args;
use std::path::PathBuf;

use crate::options::GlobalOptions;

#[derive(Debug, Args)]
pub struct WriteArgs {
    /// The path to write to
    path: PathBuf,
}

pub fn run_write(global_opts: &GlobalOptions, args: &WriteArgs) {
    println!("Running write");
    dbg!(global_opts, args);
}
