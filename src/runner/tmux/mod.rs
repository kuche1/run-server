use crate::runner::util::tmux::Tmux;

const SESSION_NAME: &str = "my-session";

/// runs the command in a tmux session
#[allow(dead_code)]
pub fn run(dir: &str, cmd: &[&str]) -> Result<(), String> {
    let tmux = Tmux::new()?;
    tmux.run(dir, SESSION_NAME, cmd, &[])
}
