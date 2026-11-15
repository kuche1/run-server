// TODO: use

use crate::util;

pub struct Tmux {}

impl Tmux {
    pub fn new() -> Result<Self, String> {
        match util::run(".", &["tmux", "-V"]) {
            Ok(()) => {}
            Err(err) => return Err(format!("Tmux is probably not installed -> {err}")),
        };

        Ok(Tmux {})
    }

    pub fn run(&self, dir: &str, args: &[&str]) -> Result<(), String> {
        util::run(dir, &[&["tmux"], args].concat())
    }
}
