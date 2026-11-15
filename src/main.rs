mod run_command;
mod runner;
mod util;

fn main() {
    let server_data_folder = &util::expand_tilde("~/server/some-server");
    let server_backup_folder = &util::expand_tilde("~/server/backup");

    //////////////////////////////////////////////////

    let runner = runner::simple::RunnerSimple::new().unwrap();

    util::backup::main(
        server_data_folder,
        util::backup::Args {
            dst: server_backup_folder.into(),
            cooldown_days: 0.9,
            history: 31,
        },
    )
    .unwrap();

    let cmd = run_command::get();
    runner.run(server_data_folder, cmd).unwrap();
}
