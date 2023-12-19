use crate::options::GlobalOptions;
use clap::Args;
use std::{error::Error, fmt::Display, process::Command};

const HELP_RUSTUP: &'static str = "make sure you have rustup installed: https://rustup.rs/";

const HELP_CARGO: &'static str =
    "make sure you have rust and cargo installed (using rustup): https://rustup.rs/";

const HELP_RUSTUP_AARCH64_MUSL_TARGET: &'static str =
    "make sure you have the target available.\ninsall it with: `rustup target add aarch64-unknown-linux-musl`";

const HELP_GNU_AARCH64_MUSL_TOOLCHAIN: &'static str =
    "make sure you have the toolchain installed. more help here: https://github.com/laurci/genezio-rs";

const HELP_GENEZIO: &'static str = "make sure you have genezio installed: https://genez.io/";

#[derive(Debug, Args)]
pub struct DoctorArgs {}

fn check_unix_based_os() -> Result<(), DoctorError> {
    if !cfg!(target_os = "linux") && !cfg!(target_os = "macos") {
        return Err(DoctorError::OS);
    }

    Ok(())
}

fn check_rustup() -> Result<(), DoctorError> {
    let status = Command::new("rustup")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|_| DoctorError::Rustup)?;

    if !status.success() {
        return Err(DoctorError::Rustup);
    }

    println!("rustup: ok");

    Ok(())
}

fn check_cargo() -> Result<(), DoctorError> {
    let status = Command::new("cargo")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|_| DoctorError::Cargo)?;

    if !status.success() {
        return Err(DoctorError::Cargo);
    }

    println!("cargo: ok");

    Ok(())
}

fn check_rustup_aarch64_musl_target() -> Result<(), DoctorError> {
    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()
        .map_err(|_| DoctorError::RustupAarch64MuslTarget)?;

    let text = String::from_utf8_lossy(&output.stdout);
    let toolchains = text.split('\n').collect::<Vec<&str>>();

    if !toolchains.contains(&"aarch64-unknown-linux-musl") {
        return Err(DoctorError::RustupAarch64MuslTarget);
    }

    println!("target aarch64-unknown-linux-musl: ok");

    Ok(())
}

fn check_gnu_aarch64_musl_toolchain() -> Result<(), DoctorError> {
    let status = Command::new("aarch64-linux-gnu-gcc")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|_| DoctorError::GnuAarch64MuslToolchain)?;

    if !status.success() {
        return Err(DoctorError::GnuAarch64MuslToolchain);
    }

    println!("toolchain aarch64-linux-musl-gnu: ok");

    Ok(())
}

fn check_genezio() -> Result<(), DoctorError> {
    let status = Command::new("genezio")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|_| DoctorError::Genezio)?;

    if !status.success() {
        return Err(DoctorError::Genezio);
    }

    println!("genezio: ok");

    Ok(())
}

pub fn run_doctor(_global_opts: &GlobalOptions, _args: &DoctorArgs) -> Result<(), DoctorError> {
    println!("Running doctor");

    check_unix_based_os()?;
    check_rustup()?;
    check_cargo()?;
    check_rustup_aarch64_musl_target()?;
    check_gnu_aarch64_musl_toolchain()?;
    check_genezio()?;

    Ok(())
}

#[derive(Debug)]
pub enum DoctorError {
    OS,
    Rustup,
    Cargo,
    RustupAarch64MuslTarget,
    GnuAarch64MuslToolchain,
    Genezio,
}

impl Display for DoctorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DoctorError: {}",
            match self {
                DoctorError::OS => "Only Linux and MacOS are supported".to_owned(),
                DoctorError::Rustup => format!("rustup not found.\nHELP: {}", HELP_RUSTUP),
                DoctorError::Cargo => format!("cargo not found.\nHELP: {}", HELP_CARGO),
                DoctorError::RustupAarch64MuslTarget => format!(
                    "aarch64-unknown-linux-musl target not found.\nHELP: {}",
                    HELP_RUSTUP_AARCH64_MUSL_TARGET
                ),
                DoctorError::GnuAarch64MuslToolchain => format!(
                    "aarch64-linux-musl-gnu toolchain not found.\nHELP: {}",
                    HELP_GNU_AARCH64_MUSL_TOOLCHAIN
                ),
                DoctorError::Genezio => format!("genezio not found.\nHELP: {}", HELP_GENEZIO),
            }
        )
    }
}

impl Error for DoctorError {}
