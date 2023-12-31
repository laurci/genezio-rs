use crate::options::GlobalOptions;
use clap::Args;
use std::{env::current_dir, error::Error, fmt::Display, fs, path::PathBuf, process::Command};

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Project name
    pub name: String,
}

fn write_readme(path: &PathBuf, name: &str) -> Result<(), NewError> {
    fs::write(
        path,
        format!(
            r#"
# {name}

Welcome to your brand new [genezio-rs](https://github.com/laurci/genezio-rs) project.

## Running & Deployment

To locally run this project you can just use `cargo run`.

To deploy this to [Genezio](https://genez.io) use `genezio-rs delpoy`. Make sure you are logged in the Genezio CLI with your account. You can check this by running `genezio account`.

After deployment you will get an HTTP API powered by `axum` at the `/` path of your service Lambda URL.

To get this URL go to the Genezio dashboard and click on your project, then click on the `Test Project` button. You will see the URL there (with some text like `Requests will be sent to https://.....`). It won't change between deploys, so you only need to do this step once.

## Doctor

If you have problems with any of the above, try running `genezio-rs doctor` in the project directory. Learn more about this [here](https://github.com/laurci/genezio-rs).

"#
        )
        .trim(),
    )
    .map_err(|_| NewError("can't write README.md".into()))?;

    Ok(())
}

fn write_gitignore(path: &PathBuf) -> Result<(), NewError> {
    fs::write(
        path,
        format!(
            r#"
/target
"#
        )
        .trim(),
    )
    .map_err(|_| NewError("can't write .gitignore".into()))?;

    Ok(())
}

fn write_genezio_yaml(path: &PathBuf, name: &str) -> Result<(), NewError> {
    fs::write(
        path,
        format!(
            r#"
name: {name}
region: eu-west-3
language: rust
cloudProvider: genezio
"#
        )
        .trim(),
    )
    .map_err(|_| NewError("can't write genezio.yaml".into()))?;

    Ok(())
}

fn write_cargo_toml(path: &PathBuf, name: &str) -> Result<(), NewError> {
    fs::write(
        path,
        format!(
            r#"
[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
genezio = {{ git = "https://github.com/laurci/genezio-rs.git" }}
"#
        )
        .trim(),
    )
    .map_err(|_| NewError("can't write Cargo.toml".into()))?;

    Ok(())
}

fn write_main_rs(path: &PathBuf, name: &str) -> Result<(), NewError> {
    fs::write(
        path,
        format!(
            r#"
use genezio::{{
    app,
    axum::{{response::Html, routing::get, Router}},
}};

async fn handler() -> Html<&'static str> {{
    Html("<h1>Hello from {name}!</h1>")
}}

#[app]
fn router() -> Router {{
    let app = Router::new().route("/", get(handler));

    app
}}
"#,
        )
        .trim(),
    )
    .map_err(|_| NewError("can't write main.rs".into()))?;

    Ok(())
}

fn write_files(target_dir: &PathBuf, name: &str) -> Result<(), NewError> {
    if !target_dir.exists() {
        std::fs::create_dir_all(target_dir)
            .map_err(|_| NewError("can't create target dir".into()))?;
    } else {
        return Err(NewError("target dir already exists".into()));
    }

    let src_dir = target_dir.join("src");
    std::fs::create_dir_all(&src_dir).map_err(|_| NewError("can't create src dir".into()))?;

    let readme = target_dir.join("README.md");
    let gitignore = target_dir.join(".gitignore");
    let genezio_yaml = target_dir.join("genezio.yaml");
    let cargo_toml = target_dir.join("Cargo.toml");
    let main_rs = src_dir.join("main.rs");

    write_readme(&readme, &name)?;
    write_gitignore(&gitignore)?;
    write_genezio_yaml(&genezio_yaml, &name)?;
    write_cargo_toml(&cargo_toml, &name)?;
    write_main_rs(&main_rs, &name)?;

    Ok(())
}

fn run_init_cmds(target_dir: &PathBuf) -> Result<(), NewError> {
    let status = Command::new("cargo")
        .arg("check")
        .current_dir(target_dir)
        .status()
        .map_err(|_| NewError("can't run cargo init".into()))?;

    if !status.success() {
        return Err(NewError("can't run cargo init".into()));
    }

    Ok(())
}

fn print_help(name: &str) {
    println!("Your project is ready!");
    println!("\nTo get started, run:");
    println!("cd ./{}", name);
    println!("genezio-rs deploy");

    println!("\nThen check your project in the genezio dashboard.");
}

fn normalize_name_to_path(name: &str) -> String {
    let name = name
        .replace("-", " ")
        .replace("_", " ")
        .chars()
        .filter(|x| x.is_alphanumeric() || *x == ' ')
        .collect::<String>();

    let comp = name
        .split(" ")
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>()
        .join("-");

    comp.trim().to_lowercase()
}

pub fn run_new(_global_opts: &GlobalOptions, args: &NewArgs) -> Result<(), NewError> {
    let name = normalize_name_to_path(&args.name);
    if name.len() == 0 {
        return Err(NewError("project name can't be empty".into()));
    }

    if name.chars().find(|_| true).unwrap().is_numeric() {
        return Err(NewError("project name can't start with a number".into()));
    }

    println!("Creating a new project in ./{}", name);
    let p = current_dir()
        .map_err(|_| NewError("can't get cwd".into()))?
        .join(&name);

    write_files(&p, &name)?;
    run_init_cmds(&p)?;
    print_help(&name);

    Ok(())
}

#[derive(Debug)]
pub struct NewError(String);

impl Display for NewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NewError: {}", self.0)
    }
}

impl Error for NewError {}
