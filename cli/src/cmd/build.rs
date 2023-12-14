use crate::{
    metadata::{get_cargo_metadata, CargoMetadata},
    options::GlobalOptions,
};
use base64::{engine::general_purpose, Engine as _};
use clap::Args;
use is_executable::IsExecutable;
use std::{error::Error, fmt::Display, fs, process::Command};

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Build in debug mode
    #[clap(long, short = 'd')]
    pub debug: bool,

    /// Clean before building
    #[clap(long, short = 'c')]
    pub clean: bool,
}

fn copy_genezio_manifest(metadata: &CargoMetadata) -> Result<(), BuildError> {
    let target_dir = metadata.get_genezio_out_dir();
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(|_| BuildError::GenezioManifestNotFound)?;
    }

    let genezio_manifest = metadata.workspace_root.join("genezio.yaml");

    if !genezio_manifest.exists() {
        return Err(BuildError::GenezioManifestNotFound);
    }

    fs::copy(genezio_manifest, target_dir.join("genezio.yaml"))
        .map_err(|_| BuildError::GenezioManifestNotFound)?;

    Ok(())
}

fn cargo_build(clean: bool, release: bool) -> Result<(), BuildError> {
    if clean {
        Command::new("cargo")
            .arg("clean")
            .status()
            .map_err(|_| BuildError::CargoBuild)?;
    }

    if release {
        Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg("aarch64-unknown-linux-musl")
            .arg("--config")
            .arg("target.aarch64-unknown-linux-musl.linker='aarch64-linux-gnu-gcc'")
            .arg("--config")
            .arg("target.aarch64-unknown-linux-musl.rustflags=[ \"-C\", \"target-feature=+crt-static\", \"-C\", \"link-arg=-lgcc\", \"--cfg\", \"genezio_with_lambda\" ]")
            .arg("--release")
            .status()
    } else {
        Command::new("cargo")
            .arg("build")
            .arg("--target")
            .arg("aarch64-unknown-linux-musl")
            .arg("--config")
            .arg("target.aarch64-unknown-linux-musl.linker='aarch64-linux-gnu-gcc'")
            .arg("--config")
            .arg("target.aarch64-unknown-linux-musl.rustflags=[ \"-C\", \"target-feature=+crt-static\", \"-C\", \"link-arg=-lgcc\", \"--cfg\", \"genezio_with_lambda\" ]")
            .status()
    }
    .map_err(|_| BuildError::CargoBuild)?;

    Ok(())
}

fn get_js_str_from_exe(bytes: &[u8]) -> Result<String, BuildError> {
    let base64 = general_purpose::STANDARD.encode(bytes);

    Ok(format!(
        "
import {{ writeFileSync, chmodSync }} from 'fs';
import {{ createRequire }} from 'module';
import {{ execSync }} from 'child_process';

const TRAP_BIN = Buffer.from('{base64}', 'base64');

@GenezioDeploy()
export class Service {{
  constructor() {{
    writeFileSync('/tmp/trap', TRAP_BIN);
    chmodSync('/tmp/trap', '755');

    console.log('trap start time', Date.now());
    execSync('/tmp/trap', {{ stdio: 'inherit' }});
  }}

  @GenezioMethod()
  async call() {{ }}
}}
"
    )
    .trim()
    .to_owned())
}

fn render_build_output(metadata: &CargoMetadata, release: bool) -> Result<(), BuildError> {
    let out_dir = metadata.get_genezio_out_dir();

    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).map_err(|e| BuildError::RenderBuildOutput(e.to_string()))?;
    }

    let release_dir = metadata.target_directory.join(if release {
        "aarch64-unknown-linux-musl/release/"
    } else {
        "aarch64-unknown-linux-musl/debug/"
    });

    if !release_dir.exists() {
        return Err(BuildError::RenderBuildOutput(
            "release directory not found".to_string(),
        ));
    }

    let Ok(release_read_dir) = fs::read_dir(release_dir) else {
        return Err(BuildError::RenderBuildOutput(
            "failed to read release directory".to_string(),
        ));
    };

    for entry in release_read_dir {
        let entry = entry.map_err(|e| BuildError::RenderBuildOutput(e.to_string()))?;
        let path = entry.path();
        if path.is_file() && path.is_executable() {
            let in_bytes =
                fs::read(path.clone()).map_err(|e| BuildError::RenderBuildOutput(e.to_string()))?;

            let out_path = out_dir.join("index.js");
            let out_str = get_js_str_from_exe(&in_bytes)?;
            fs::write(out_path, out_str)
                .map_err(|e| BuildError::RenderBuildOutput(e.to_string()))?;

            break;
        }
    }

    Ok(())
}

pub fn run_build(_global_opts: &GlobalOptions, args: &BuildArgs) -> Result<(), BuildError> {
    println!("Starting build");

    let metadata = get_cargo_metadata().map_err(|e| BuildError::Metadata(e.to_string()))?;
    copy_genezio_manifest(&metadata)?;
    cargo_build(args.clean, !args.debug)?;
    render_build_output(&metadata, !args.debug)?;

    println!("Build finished");

    Ok(())
}

#[derive(Debug)]
pub enum BuildError {
    Metadata(String),
    CargoBuild,
    RenderBuildOutput(String),
    GenezioManifestNotFound,
}

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BuildError: {}",
            match self {
                BuildError::Metadata(text) => format!("failed to get cargo metadata: {text}"),
                BuildError::GenezioManifestNotFound =>
                    "genezio.yaml not found in workspace root".to_string(),
                BuildError::CargoBuild => "failed to build with cargo".to_string(),
                BuildError::RenderBuildOutput(text) =>
                    format!("failed to render build output: {text}"),
            }
        )
    }
}

impl Error for BuildError {}
