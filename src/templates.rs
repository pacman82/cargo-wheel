use serde_derive::Serialize;
use std::{fs::File, path::Path};

const INIT_PY: &str = include_str!("__init__.py.mustache");
const SETUP_PY: &str = include_str!("setup.py.mustache");

#[cfg(not(target_os = "windows"))]
const EXECUTABLE_FILE_ENDING: &str = "";

#[cfg(target_os = "windows")]
const EXECUTABLE_FILE_ENDING: &str = ".exe";

#[derive(Serialize)]
pub struct SetupPyVars<'a> {
    name: &'a str,
    c_dylib: &'a str,
    version: &'a str,
    url: &'a str,
    author: &'a str,
    description: &'a str,
    executable_file_ending: &'a str,
    crate_dir: &'a str,
}

impl<'a> SetupPyVars<'a> {
    pub fn new(
        name: &'a str,
        c_dylib: &'a str,
        version: &'a str,
        url: &'a str,
        author: &'a str,
        description: &'a str,
        crate_dir: &'a str,
    ) -> Self {
        SetupPyVars {
            name,
            c_dylib,
            version,
            url,
            author,
            description,
            executable_file_ending: EXECUTABLE_FILE_ENDING,
            crate_dir,
        }
    }
}

#[derive(Serialize)]
struct InitPyVars<'a> {
    name: &'a str,
}

pub fn render_setup_py(
    path: &Path,
    setup_py_vars: SetupPyVars<'_>,
) {
    let template = mustache::compile_str(SETUP_PY).unwrap();
    let mut file = File::create(path).expect("Unable to create setup.py");
    template
        .render(
            &mut file,
            &setup_py_vars,
        )
        .expect("Failed rendering setup.py");
}

pub fn render_init_py(path: &Path, name: &str) {
    let template = mustache::compile_str(INIT_PY).unwrap();
    let mut file = File::create(path).expect("Unable to create __init__.py");
    template
        .render(&mut file, &InitPyVars { name })
        .expect("Failed rendering __init__.py");
}
