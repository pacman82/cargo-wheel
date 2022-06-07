mod header;
mod templates;

use cargo::{
    core::{shell::Shell, Workspace},
    util::important_paths,
    CliResult, Config,
};
use std::{
    env::current_dir,
    fs::create_dir_all,
    path::{Path, PathBuf},
    process::Command,
};
use structopt::{clap::AppSettings, StructOpt};

use crate::templates::SetupPyVars;

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
enum Opts {
    #[structopt(
        name = "wheel",
        setting = AppSettings::UnifiedHelpMessage,
        setting = AppSettings::DeriveDisplayOrder,
        setting = AppSettings::DontCollapseArgsInUsage
    )]
    /// Package executable as a python wheel
    Wheel(Args),
}

#[derive(StructOpt)]
struct Args {
    /// Directory for all generated artifacts
    #[structopt(long = "target-dir", value_name = "DIRECTORY", parse(from_os_str))]
    target_dir: Option<PathBuf>,
    #[structopt(long, short = "v", parse(from_occurrences))]
    /// Use verbose output (-vv very verbose/build.rs output)
    verbose: u32,
    #[structopt(long, short = "q")]
    /// No output printed to stdout other than the tree
    quiet: bool,
    #[structopt(long, value_name = "WHEN")]
    /// Coloring: auto, always, never
    color: Option<String>,
    #[structopt(long)]
    /// Require Cargo.lock and cache are up to date
    frozen: bool,
    #[structopt(long)]
    /// Require Cargo.lock is up to date
    locked: bool,
    /// Run without accessing the network
    #[structopt(long)]
    offline: bool,
    #[structopt(short = "Z", value_name = "FLAG")]
    /// Unstable (nightly-only) flags to Cargo
    unstable_flags: Vec<String>,
}

fn main() {
    env_logger::init();

    let mut config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => {
            let mut shell = Shell::new();
            cargo::exit_with_error(e.into(), &mut shell)
        }
    };

    let Opts::Wheel(args) = Opts::from_args();

    if let Err(err) = real_main(args, &mut config) {
        let mut shell = Shell::new();
        cargo::exit_with_error(err, &mut shell)
    }
}

fn real_main(args: Args, config: &mut Config) -> CliResult {
    let cli_config = [];

    config.configure(
        args.verbose,
        args.quiet,
        args.color.as_deref(),
        args.frozen,
        args.locked,
        args.offline,
        &args.target_dir,
        &args.unstable_flags,
        &cli_config,
    )?;

    let manifest_path = important_paths::find_root_manifest_for_wd(config.cwd())?;
    let workspace = Workspace::new(&manifest_path, config)?;
    let package = workspace.current()?;
    let crate_dir = manifest_path
        .parent()
        .expect("Expected manifest path to point to a file");

    let setup_py_dir = Path::new(".");
    let absolute_setup_py_dir = current_dir()
        .expect("Error determining working directory")
        .join(setup_py_dir);
    let relative_crate_dir = pathdiff::diff_paths(crate_dir, &absolute_setup_py_dir)
        .expect("Could not determine crate directory relative to directory containing setup.py");
    let relative_crate_dir = if relative_crate_dir == Path::new("") {
        PathBuf::from(".")
    } else {
        relative_crate_dir
    };
    let setup_py_path = setup_py_dir.join("setup.py");
    let py_package_name = &package.name().replace('-', "_");
    let py_package_dir = setup_py_dir.join(&py_package_name);
    let c_dylib_name = package
        .targets()
        .iter()
        .find(|t| t.is_cdylib())
        .expect(
            "No dynamic C-Library found in targets. Do you miss:\
             \n\
             \n[lib]\
             \ncrate-type = [\"cdylib\"]\
             \n\
             \nin your Cargo.toml?",
        )
        .crate_name();

    println!("Generate C Header file");
    header::generate_c_bindings(crate_dir, py_package_name);

    if !setup_py_path.exists() {
        let version = package.version().to_string();
        let setup_py_vars = SetupPyVars::new(
            py_package_name,
            &c_dylib_name,
            &version,
            "url",
            "authors",
            "description",
            relative_crate_dir
                .to_str()
                .expect("Crate path contains invalid unicode characters."),
        );
        templates::render_setup_py(&setup_py_path, setup_py_vars);
    }

    // Assert python package direcotry
    create_dir_all(&py_package_dir).expect("Error creating python package directory");
    // Assert existing __init__.py
    let init_py_path = py_package_dir.join("__init__.py");
    if !init_py_path.exists() {
        templates::render_init_py(&init_py_path, py_package_name);
    }

    println!("python setup.py bdist_wheel");
    let exit_code = Command::new("python")
        .arg("setup.py")
        .arg("bdist_wheel")
        .status()
        .expect("Error executing 'python setup.py bdist_wheel'");

    println!("'python setup.py bdist_wheel' finished: {}", exit_code);

    Ok(())
}
