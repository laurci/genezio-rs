use crate::{
    cmd::build::{run_build, BuildArgs},
    metadata::get_cargo_metadata,
    options::GlobalOptions,
};
use clap::Args;
use std::{error::Error, fmt::Display, process::Command};

use super::build::BuildError;

#[derive(Debug, Args)]
pub struct DeployArgs {
    /// Build in debug mode
    #[clap(long, short = 'd')]
    pub debug: bool,

    /// Clean before building
    #[clap(long, short = 'c')]
    pub clean: bool,
}

impl From<&DeployArgs> for BuildArgs {
    fn from(args: &DeployArgs) -> Self {
        Self {
            debug: args.debug,
            clean: args.clean,
        }
    }
}

fn run_genezio_deploy(metadata: &crate::metadata::CargoMetadata) -> Result<(), DeployError> {
    let status = Command::new("genezio")
        .arg("deploy")
        .current_dir(metadata.get_genezio_out_dir())
        .status()
        .map_err(|_| DeployError::Genezio)?;

    if !status.success() {
        return Err(DeployError::Genezio);
    }

    Ok(())
}

pub fn run_deploy(global_opts: &GlobalOptions, args: &DeployArgs) -> Result<(), DeployError> {
    println!("Starting deploy");

    let metadata = get_cargo_metadata().map_err(|e| DeployError::Metadata(e.to_string()))?;
    run_build(global_opts, &args.into()).map_err(|e| DeployError::BuildError(e))?;
    run_genezio_deploy(&metadata)?;

    println!("Deploy finished");

    Ok(())
}

#[derive(Debug)]
pub enum DeployError {
    Metadata(String),
    BuildError(BuildError),
    Genezio,
}

impl Display for DeployError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeployError: {}",
            match self {
                DeployError::Metadata(text) => format!("failed to get cargo metadata: {text}"),
                DeployError::BuildError(err) => format!("failed to build: {err}"),
                DeployError::Genezio => format!("failed to deploy to genezio"),
            }
        )
    }
}

impl Error for DeployError {}
