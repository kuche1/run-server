mod config;

use std::env;
use std::process::Command;

fn main() {
    let dir = shellexpand::tilde(config::WORKING_DIRECTORY);
    env::set_current_dir(dir.as_ref()).unwrap();

    Command::new(config::RUN_SERVER[0])
        .args(&config::RUN_SERVER[1..])
        .status()
        .unwrap();
}
