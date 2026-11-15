mod config;

use std::env;

fn main() {
    let dir = shellexpand::tilde(config::WORKING_DIRECTORY);
    env::set_current_dir(dir.as_ref()).unwrap();
}
