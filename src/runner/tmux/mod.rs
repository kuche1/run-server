use crate::runner::util::tmux::Tmux;

const SESSION_NAME: &str = "my-session";

pub struct RunnerTmux {
    tmux: Tmux,
}

/// runs the command in a tmux session
impl RunnerTmux {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, String> {
        Ok(RunnerTmux { tmux: Tmux::new()? })
    }

    #[allow(dead_code)]
    pub fn run(&self, dir: &str, cmd: &[&str]) -> Result<(), String> {
        self.tmux.run(dir, SESSION_NAME, cmd, &[])
    }
}
