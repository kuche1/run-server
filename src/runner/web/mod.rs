mod ttyd;

use crate::runner::util::tmux::Tmux;
use ttyd::Ttyd;

const SESSION_NAME: &str = "my-session";

pub struct RunnerWeb {
    tmux: Tmux,
    ttyd: Ttyd,
    web_interface_port: u16,
    web_interface_username: String,
    web_interface_password: String,
}

/// runs the command in a tmux session, and makes it available in the browser via ttyd
impl RunnerWeb {
    #[allow(dead_code)]
    pub fn new(
        web_interface_port: u16,
        web_interface_username: &str,
        web_interface_password: &str,
    ) -> Result<Self, String> {
        Ok(RunnerWeb {
            tmux: Tmux::new()?,
            ttyd: Ttyd::new()?,
            web_interface_port,
            web_interface_username: web_interface_username.to_string(),
            web_interface_password: web_interface_password.to_string(),
        })
    }

    #[allow(dead_code)]
    pub fn run(&self, dir: &str, cmd: &[&str]) -> Result<(), String> {
        let _kill_spawned_ttyd = self.ttyd.spawn(
            self.web_interface_port,
            &self.web_interface_username,
            &self.web_interface_password,
            SESSION_NAME,
        )?;

        self.tmux.run(
            dir,
            SESSION_NAME,
            cmd,
            // disable ctrl+b -> trying to handicap tmux so that the connected user cannot break
            // out of the server terminal (this is not really necessary but its nice to have)
            &["set", "-g", "prefix", "None"],
        )
    }
}
