use std::process::Command;

pub fn run(dir: &str, cmd: &[&str]) {
    Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(dir)
        .status()
        .unwrap();
}

pub fn expand_tilde(path: &str) -> String {
    shellexpand::tilde(path).into_owned()
}
