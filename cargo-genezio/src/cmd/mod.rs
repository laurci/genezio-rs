use clap::Subcommand;

use crate::options::GlobalOptions;

mod read;
mod write;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Help message for read.
    Read(read::ReadArgs),
    /// Help message for write.
    Write(write::WriteArgs),
}

impl Command {
    pub fn run(&self, global_opts: &GlobalOptions) {
        match self {
            Command::Read(args) => read::run_read(global_opts, args),
            Command::Write(args) => write::run_write(global_opts, args),
        }
    }
}
