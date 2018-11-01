extern crate cargo;
extern crate env_logger;
extern crate structopt;

mod setup_py;
use cargo::{
    core::{shell::Shell, Workspace},
    util::important_paths,
    CliResult, Config,
};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};
use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
enum Opts {
    #[structopt(
        name = "wheel",
        raw(
            setting = "AppSettings::UnifiedHelpMessage",
            setting = "AppSettings::DeriveDisplayOrder",
            setting = "AppSettings::DontCollapseArgsInUsage"
        )
    )]
    /// Package executable as a python wheel
    Wheel(Args),
}

#[derive(StructOpt)]
struct Args {
    /// Directory for all generated artifacts
    #[structopt(
        long = "target-dir",
        value_name = "DIRECTORY",
        parse(from_os_str)
    )]
    target_dir: Option<PathBuf>,
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    /// Use verbose output (-vv very verbose/build.rs output)
    verbose: u32,
    #[structopt(long = "quiet", short = "q")]
    /// No output printed to stdout other than the tree
    quiet: Option<bool>,
    #[structopt(long = "color", value_name = "WHEN")]
    /// Coloring: auto, always, never
    color: Option<String>,
    #[structopt(long = "frozen")]
    /// Require Cargo.lock and cache are up to date
    frozen: bool,
    #[structopt(long = "locked")]
    /// Require Cargo.lock is up to date
    locked: bool,
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

    if let Err(e) = real_main(args, &mut config) {
        let mut shell = Shell::new();
        cargo::exit_with_error(e.into(), &mut shell)
    }
}

fn real_main(args: Args, config: &mut Config) -> CliResult {
    config.configure(
        args.verbose,
        args.quiet,
        &args.color,
        args.frozen,
        args.locked,
        &args.target_dir,
        &args.unstable_flags,
    )?;

    let manifest_path = important_paths::find_root_manifest_for_wd(config.cwd())?;
    let workspace = Workspace::new(&manifest_path, config)?;
    let package = workspace.current()?;

    if !Path::new("setup.py").exists() {
        let mut file = File::create("setup.py").expect("Unable to create setup.py");
        file.write_all(
            setup_py::render_with(
                &package.name(),
                &package.version().to_string(),
                "url",
                "authors",
                "description",
            ).as_bytes(),
        ).expect("Unable to write setup.py");
    }

    let out = Command::new("python")
        .arg("setup.py")
        .arg("bdist_wheel")
        .output()
        .expect("Error executing python setup.py bdist_wheel");

    println!(
        "bdist_wheel stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    println!(
        "bdist_wheel stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    Ok(())
}
