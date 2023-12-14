use clap::Subcommand;

use crate::options::GlobalOptions;

mod build;
mod deploy;
mod doctor;
mod new;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new project
    New(new::NewArgs),

    /// Build the project
    Build(build::BuildArgs),

    /// Deploy the project to genezio
    Deploy(deploy::DeployArgs),

    /// Verify all dependencies
    Doctor(doctor::DoctorArgs),
}

impl Command {
    pub fn run(&self, global_opts: &GlobalOptions) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::New(args) => new::run_new(global_opts, args).map_err(|e| e.into()),
            Command::Build(args) => build::run_build(global_opts, args).map_err(|e| e.into()),
            Command::Deploy(args) => deploy::run_deploy(global_opts, args).map_err(|e| e.into()),
            Command::Doctor(args) => doctor::run_doctor(global_opts, args).map_err(|e| e.into()),
        }
    }
}
