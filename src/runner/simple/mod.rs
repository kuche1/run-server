use crate::util;

#[allow(dead_code)]
pub struct RunnerSimple {}

/// runs the command directly in the current terminal
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
