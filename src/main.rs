mod api;
mod api_token;
mod app;
mod exit_codes;
mod prompt;
mod subcommand;

use anyhow::{anyhow, Result};
use std::process;

fn run() -> Result<exit_codes::ExitCode> {
    let matches = app::build_app().get_matches();

    if let (subcommand_name, Some(_)) = matches.subcommand() {
        match subcommand_name {
            "login" => {
                return subcommand::login();
            }
            "clock_in" => {
                return subcommand::clock_in();
            }
            "clock_out" => {
                return subcommand::clock_out();
            }
            "start_break" => {
                return subcommand::start_break();
            }
            "end_break" => {
                return subcommand::end_break();
            }
            _ => {
                return Err(anyhow!(format!(
                    "Unknown subcommand given. {}",
                    subcommand_name
                )));
            }
        }
    } else {
        Err(anyhow!("subcommand not given"))
    }
}

fn main() {
    match run() {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(error) => {
            eprintln!("[error]: {}", error);
            app::build_app().print_help().unwrap();
            process::exit(exit_codes::ExitCode::Failure.into());
        }
    }
}
