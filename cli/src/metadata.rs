use serde::Deserialize;
use std::{path::PathBuf, process::Command};

#[derive(Debug, Deserialize)]
pub struct CargoMetadata {
    pub target_directory: PathBuf,
    pub workspace_root: PathBuf,
}

impl CargoMetadata {
    pub fn get_genezio_out_dir(&self) -> PathBuf {
        self.target_directory.join("genezio/out")
    }
}

pub fn get_cargo_metadata() -> Result<CargoMetadata, Box<dyn std::error::Error>> {
    let text = Command::new("cargo").arg("metadata").output()?.stdout;
    Ok(serde_json::from_slice(&text)?)
}
