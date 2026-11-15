mod tmux;
mod ttyd;

use tmux::Tmux;
use ttyd::Ttyd;

/// runs the command in a tmux session, and makes it available in the browser via ttyd
#[allow(dead_code)]
pub fn run(
    dir: &str,
    cmd: &[&str],
    web_interface_port: u16,
    web_interface_username: &str,
    web_interface_password: &str,
) -> Result<(), String> {
    let tmux = Tmux::new()?;
    let ttyd = Ttyd::new()?;

    let _kill_spawned_ttyd = ttyd.spawn(
        web_interface_port,
        web_interface_username,
        web_interface_password,
    )?;

    tmux.run(dir, cmd)
}
