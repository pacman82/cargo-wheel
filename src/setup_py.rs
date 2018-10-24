const SETUP_PY: &str = include_str!("setup.py");

#[cfg(not(target_os="windows"))]
const EXECUTABLE_FILE_ENDING : &str = "";

#[cfg(target_os="windows")]
const EXECUTABLE_FILE_ENDING : &str = ".exe";

pub fn render_with(
    name: &str,
    version: &str,
    url: &str,
    author: &str,
    description: &str,
) -> String {
    SETUP_PY
        .replace("{{{name}}}", name)
        .replace("{{{version}}}", version)
        .replace("{{{url}}}", url)
        .replace("{{{author}}}", author)
        .replace("{{{description}}}", description)
        .replace("{{{executable_file_ending}}}", EXECUTABLE_FILE_ENDING)
}
