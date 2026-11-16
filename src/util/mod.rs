pub mod backup;
pub mod backup_v2;

use std::process::Command;

pub fn run(dir: &str, cmd: &[&str]) -> Result<(), String> {
    let status = match Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(dir)
        .status()
    {
        Ok(status) => status,
        Err(err) => return Err(format!("{err} -> {cmd:?}")),
    };

    if !status.success() {
        return Err(format!("{status} -> {cmd:?}"));
        // examples what `{status}` could translate to:
        // exit status: 1
        // signal: 9 (SIGKILL)
    }

    Ok(())
}

pub fn expand_tilde(path: &str) -> String {
    shellexpand::tilde(path).into_owned()
}
