// TODO: The web interface can open a full shell. Anyone logged into ttyd can press Ctrl+b c in tmux and get a shell as your server user, not just the server console. The no-bindings tmux config from earlier (-L + unbind -a) blocks that.
// TODO: Other users on the machine can see the password. --credential asd:fgh is a command-line argument, so ps -ef shows it to every user.

mod backup;
mod tmux;
mod ttyd;
mod util;

fn main() {
    let server_port = 6969;

    let web_interface_port = 6970;
    let web_interface_username = "asd";
    let web_interface_password = "fgh";

    let server_data_folder = &util::expand_tilde("~/server/some-server");
    let backup_folder = &util::expand_tilde("~/server/backup");

    //////////////////////////////////////////////////

    let tmux = tmux::Tmux::new().unwrap();
    let ttyd = ttyd::Ttyd::new().unwrap();

    backup::main(
        server_data_folder,
        backup::Args {
            dst: backup_folder.into(),
            cooldown_days: 0.9,
            history: 31,
        },
    )
    .unwrap();

    let _kill_spawned_ttyd = ttyd
        .spawn(
            web_interface_port,
            web_interface_username,
            web_interface_password,
        )
        .unwrap();

    tmux.run(
        server_data_folder,
        &[
            "cargo",
            "run",
            "--",
            "--addr",
            &format!("0.0.0.0:{server_port}"),
        ],
    )
    .unwrap();
}
