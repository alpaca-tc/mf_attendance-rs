use clap::{crate_version, App};

pub fn build_app() -> App<'static, 'static> {
    App::new("mf_attendance")
        .version(crate_version!())
        .subcommand(build_subcommand_login())
        .subcommand(build_subcommand_clock_in())
        .subcommand(build_subcommand_clock_out())
        .subcommand(build_subcommand_start_break())
        .subcommand(build_subcommand_end_break())
}

fn build_subcommand_login() -> App<'static, 'static> {
    App::new("login")
}

fn build_subcommand_clock_in() -> App<'static, 'static> {
    App::new("clock_in")
}

fn build_subcommand_clock_out() -> App<'static, 'static> {
    App::new("clock_out")
}

fn build_subcommand_start_break() -> App<'static, 'static> {
    App::new("start_break")
}

fn build_subcommand_end_break() -> App<'static, 'static> {
    App::new("end_break")
}
