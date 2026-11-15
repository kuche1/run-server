mod ttyd;

use crate::runner::util::tmux::Tmux;
use ttyd::Ttyd;

const SESSION_NAME: &str = "my-session";

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
        SESSION_NAME,
    )?;

    tmux.run(
        dir,
        SESSION_NAME,
        cmd,
        // disable ctrl+b -> trying to handicap tmux so that the connected user cannot break
        // out of the server terminal (this is not really necessary but its nice to have)
        &["set", "-g", "prefix", "None"],
    )
}
