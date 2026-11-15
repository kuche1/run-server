use crate::util;

/// Runs the command directly in the current terminal
pub struct RunnerSimple {}

impl RunnerSimple {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, String> {
        Ok(RunnerSimple {})
    }

    #[allow(dead_code)]
    pub fn run(&self, dir: &str, cmd: &[&str]) -> Result<(), String> {
        util::run(dir, cmd)
    }
}
