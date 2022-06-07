use cbindgen::{generate_with_config, Config, Language};
use std::path::Path;

/// Generates c-header file.
pub fn generate_c_bindings(manifest_dir: &Path, crate_name: &str) {
    let config = Config {
        language: Language::C,
        ..Default::default()
    };
    generate_with_config(manifest_dir, config)
        .expect("Error generating C header file.")
        .write_to_file(format!("target/{}.h", crate_name));
}
