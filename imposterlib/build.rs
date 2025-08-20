use std::{error::Error, path::PathBuf, process::exit};

fn main() {
    if let Err(err) = main_with_error() {
        eprintln!("fatal: {err}");
        exit(1);
    };
}

fn main_with_error() -> Result<(), Box<dyn Error>> {
    let project_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    let project_dir = PathBuf::from(project_dir);
    if !project_dir.exists() {
        Err(format!(
            "project dir does not exist at: '{}'",
            project_dir.display()
        ))?
    }

    let mut def_file = project_dir.clone();
    def_file.push("targetlib.def");

    if !def_file.exists() {
        Err(format!(
            "def file does not exist at: '{}'",
            def_file.display()
        ))?
    }

    let mut build_opts = String::new();
    build_opts.push_str("cargo::rustc-link-arg-cdylib=/DEF:");
    build_opts.push_str(def_file.display().to_string().as_str());

    println!("{build_opts}");

    Ok(())
}
