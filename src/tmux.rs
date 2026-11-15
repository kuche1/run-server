use crate::util;
use std::os::unix::fs::OpenOptionsExt;

const SESSION_NAME: &str = "my-session";
// TODO: use in ttyd

const ENV_VAR_USER: &str = "USER";

pub struct Tmux {}

impl Tmux {
    pub fn new() -> Result<Self, String> {
        match util::run(".", &["tmux", "-V"]) {
            Ok(()) => {}
            Err(err) => return Err(format!("Tmux is probably not installed -> {err}")),
        };

        Ok(Tmux {})
    }

    pub fn run(&self, dir: &str, cmd: &[&str]) -> Result<(), String> {
        let user = match std::env::var(ENV_VAR_USER) {
            Ok(ok) => ok,
            Err(err) => return Err(format!("{err} -> {ENV_VAR_USER}")),
        };

        let file_return_code = &format!("/tmp/{user}_server-return-code");

        // this may prove to be a bad idea
        // in that case it can be removed inthe future
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600) // only this user can read/write to this file
            .open(file_return_code)
        {
            Ok(_) => {}
            Err(err) => {
                return Err(format!(
                    "Probably the server is already running or has been brutally interrupted -> {err} -> {file_return_code}"
                ));
            }
        }

        // deletes the file when the function returns
        // if the variable name is set to `_` its going to get removed immediately instead
        let _delete_file_return_code = DeleteOnDrop(file_return_code);

        let script_get_return_code = format!("\"$@\"; echo $? > {file_return_code}");

        util::run(
            dir,
            &[
                &[
                    "tmux",
                    "new",
                    "-s",
                    SESSION_NAME,
                    "--",
                    "sh",
                    "-c",
                    &script_get_return_code,
                    "sh",
                ],
                cmd,
            ]
            .concat(),
        )?;

        let code = match std::fs::read_to_string(file_return_code) {
            Ok(code) => code,
            Err(err) => return Err(format!("{err} -> {file_return_code}")),
        };

        let code = code.trim();

        match code {
            "0" => Ok(()),
            "" => Err("Return code not set".to_string()),
            code => Err(format!("Bad return code -> {code} -> {cmd:?}")),
        }
    }
}

struct DeleteOnDrop<'a>(&'a str);

impl Drop for DeleteOnDrop<'_> {
    fn drop(&mut self) {
        match std::fs::remove_file(self.0) {
            Ok(()) => {}
            Err(err) => eprintln!("Could not delete file -> {err} -> {}", self.0),
        }
    }
}
