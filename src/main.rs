mod backup;
mod tmux;
mod util;

fn main() {
    let server_data_folder = &util::expand_tilde("~/server/some-server");
    let backup_folder = &util::expand_tilde("~/server/backup");

    let tmux = tmux::Tmux::new().unwrap();

    backup::main(
        server_data_folder,
        backup::Args {
            dst: backup_folder.into(),
            cooldown_days: 0.9,
            history: 31,
        },
    )
    .unwrap();

    util::run(
        server_data_folder,
        &["cargo", "run", "--", "--addr", "0.0.0.0:6969"],
    )
    .unwrap();
}
