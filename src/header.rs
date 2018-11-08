use cbindgen;
use std::path::Path;

/// Generates c-header file.
pub fn generate_c_bindings(manifest_dir: &Path, crate_name: &str) {
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;
    cbindgen::generate_with_config(manifest_dir, config)
        .expect("Error generating C header file.")
        .write_to_file(format!("target/{}.h", crate_name));
}
