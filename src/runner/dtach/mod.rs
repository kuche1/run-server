// TODO: check if the foreground stuff really does not work

use crate::util;

const SOCKET: &str = "dtach.sock";

/// Runs the command headless with dtach
/// Attach with `dtach -a <dtach-sock-file>`
/// Detach with `Ctrl+\`
pub struct RunnerDtach {}

impl RunnerDtach {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, String> {
        match util::run(".", &["dtach", "--version"]) {
            Ok(()) => {}
            Err(err) => return Err(format!("Dtach is probably not installed -> {err}")),
        };

        Ok(RunnerDtach {})
    }

    #[allow(dead_code)]
    pub fn run(&self, dir: &str, cmd: &[&str]) -> Result<(), String> {
        let cwd = match std::env::current_dir() {
            Ok(ok) => ok,
            Err(err) => return Err(format!("Could not get current directory -> {err}")),
        };

        // absolute path since the command below runs in `dir` and not in cwd
        let socket = cwd.join(SOCKET).display().to_string();

        // dtach does return the command's exit code (unlike fucking tmux)
        // `-N` -> new session, does not daemonize, blocks until the command finishes
        // `-c` does not work if running as a systemd service
        util::run(dir, &[&["dtach", "-N", &socket], cmd].concat())
    }
}
