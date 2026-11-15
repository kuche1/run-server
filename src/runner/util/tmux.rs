use crate::util;
use std::process::Command;

pub struct Tmux {}

impl Tmux {
    pub fn new() -> Result<Self, String> {
        match util::run(".", &["tmux", "-V"]) {
            Ok(()) => {}
            Err(err) => return Err(format!("Tmux is probably not installed -> {err}")),
        };

        Ok(Tmux {})
    }

    fn run_basic(&self, dir: &str, args: &[&str]) -> Result<(), String> {
        util::run(dir, &[&["tmux"], args].concat())
    }

    fn output(&self, args: &[&str]) -> Result<String, String> {
        // runs `tmux <args>` and returns what it printed

        let output = match Command::new("tmux").args(args).output() {
            Ok(output) => output,
            Err(err) => return Err(format!("{err} -> tmux {args:?}")),
        };

        if !output.status.success() {
            return Err(format!(
                "Bad return code -> {} -> tmux {args:?} -> {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn run(
        &self,
        dir: &str,
        session_name: &str,
        cmd: &[&str],
        additional_cmd_after_session_creation: &[&str],
    ) -> Result<(), String> {
        let channel_command_finished = format!("{session_name}_command-finished");

        // without this tmux could exit, and the code would be lost
        let channel_exit_code_received = format!("{session_name}_exit-code-received");

        let vatiable_exit_code = format!("@{session_name}_exit-code");

        let bash_glue_bullshit_fuck = format!(
            "\"$@\"; tmux set -g {vatiable_exit_code} $?; tmux wait-for -S {channel_command_finished}; tmux wait-for {channel_exit_code_received}"
        );
        //
        ////// this may (should) become an issue again in the future but i will keep it commented out for now
        // `trap : INT` so that ctrl+c only kills the command, and not `bash` (otherwise the exit code is never set)
        // add `trap : INT; ` to the start of `script`

        let mut args = vec![
            "new",
            "-d", // detached, so that it also works without a terminal (systemd)
            "-s",
            session_name,
            "--",
            "bash",
            "-c",
            &bash_glue_bullshit_fuck,
            "bash",
        ];

        args.extend(cmd);

        if !additional_cmd_after_session_creation.is_empty() {
            args.push(";");
            args.extend(additional_cmd_after_session_creation);
        }

        self.run_basic(dir, &args)?;

        // block until the we get the "done" signal
        self.run_basic(".", &["wait-for", &channel_command_finished])?;

        // get the exit code
        let exit_code = self.output(&["show", "-gv", &vatiable_exit_code]);

        // send confirmation that we have gotten the exit code
        self.run_basic(".", &["wait-for", "-S", &channel_exit_code_received])?;

        match exit_code?.as_str() {
            "0" => Ok(()),
            code => Err(format!("Bad return code -> {code} -> {cmd:?}")),
        }
    }
}
