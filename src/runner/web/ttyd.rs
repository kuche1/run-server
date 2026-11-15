// TODO: Other users on the machine can see the password. --credential asd:fgh is a command-line argument, so ps -ef shows it to every user.

use crate::util;
use std::process::{Child, Command, Stdio};

pub struct Ttyd {}

impl Ttyd {
    pub fn new() -> Result<Self, String> {
        match util::run(".", &["ttyd", "--version"]) {
            Ok(()) => {}
            Err(err) => return Err(format!("Ttyd is probably not installed -> {err}")),
        };

        Ok(Ttyd {})
    }

    pub fn spawn(&self, port: u16, username: &str, password: &str, tmux_session: &str) -> Result<TtydProcess, String> {
        let port = port.to_string();
        let credentials = format!("{username}:{password}");

        let tmux_session = format!("={tmux_session}");
        // `=` means exact name match

        let cmd = [
            "ttyd",
            "--port",
            &port,
            "--credential",
            &credentials,
            // browser seesion can write to terminal
            "--writable",
            // no automatic reconnect in browser
            "--client-option",
            "disableReconnect=true",
            //
            "--",
            "tmux",
            "attach",
            "-t",
            &tmux_session,
        ];

        // output goes to /dev/null, otherwise ttyd's logs get printed over the tmux session
        match Command::new(cmd[0])
            .args(&cmd[1..])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => Ok(TtydProcess(child)),
            Err(err) => Err(format!("{err} -> {cmd:?}")),
        }
    }
}

/// kills ttyd when dropped
pub struct TtydProcess(Child);

impl Drop for TtydProcess {
    fn drop(&mut self) {
        match self.0.kill() {
            Ok(()) => {}
            Err(err) => eprintln!("Could not kill ttyd -> {err}"),
        }

        // wait so that it does not stay around as a zombie process
        match self.0.wait() {
            Ok(_) => {}
            Err(err) => eprintln!("Could not wait for ttyd -> {err}"),
        }
    }
}
