mod backup;
mod util;

fn main() {
    let server_data_folder = &util::expand_tilde("~/server/some-server");
    let backup_folder = &util::expand_tilde("~/server/backup");

    backup::main(backup::Args {
        src: server_data_folder.into(),
        dst: backup_folder.into(),
        cooldown_days: 0.9,
        history: 31,
    })
    .unwrap();

    util::run(
        server_data_folder,
        &["cargo", "run", "--", "--addr", "0.0.0.0:6969"],
    );
}
