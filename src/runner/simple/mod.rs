use crate::util;

/// runs the command directly in the current terminal
#[allow(dead_code)]
pub fn run(dir: &str, cmd: &[&str]) -> Result<(), String> {
    util::run(dir, cmd)
}
