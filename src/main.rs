mod backup;
mod runner;
mod util;

fn main() {
    let server_port = 6969;

    let server_data_folder = &util::expand_tilde("~/server/some-server");
    let backup_folder = &util::expand_tilde("~/server/backup");

    //////////////////////////////////////////////////

    let runner = runner::simple::RunnerSimple::new().unwrap();

    backup::main(
        server_data_folder,
        backup::Args {
            dst: backup_folder.into(),
            cooldown_days: 0.9,
            history: 31,
        },
    )
    .unwrap();

    let cmd = &[
        "cargo",
        "run",
        "--",
        "--addr",
        &format!("0.0.0.0:{server_port}"),
    ];

    runner.run(server_data_folder, cmd).unwrap();
    //
    // runner::tmux_ttyd::run(
    //     server_data_folder,
    //     cmd,
    //     6970,
    //     "asd",
    //     "fgh",
    // )
    // .unwrap();
}
