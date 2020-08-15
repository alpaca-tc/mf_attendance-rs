use crate::api;
use crate::api_token;
use crate::exit_codes;
use crate::prompt;
use anyhow::{anyhow, Result};
use chrono::Utc;

fn now() -> String {
    let utc = Utc::now();
    utc.format("%Y-%m-%dT%H:%M:%S%z").to_string()
}

fn create_attendance_record(event: &str) -> Result<exit_codes::ExitCode> {
    if api_token::api_token_exists() {
        let now = now();
        let response = api::create_attendance_record(event, &*now)?;

        if response.status() == 200 || response.status() == 409 {
            println!("done");
            Ok(exit_codes::ExitCode::Success)
        } else {
            Err(anyhow!(response.text()?))
        }
    } else {
        Err(anyhow!("Please login."))
    }
}

pub fn login() -> Result<exit_codes::ExitCode> {
    prompt::echo("Copy your api token from https://attendance.moneyforward.com/my_page/settings/employee_api_token\n")?;
    let api_token = prompt::ask("Paste your api token: ")?;

    api_token::create_api_token(api_token.as_bytes())?;

    prompt::echo("Store token to ~/.config/mf_attendance\n")?;

    Ok(exit_codes::ExitCode::Success)
}

pub fn clock_in() -> Result<exit_codes::ExitCode> {
    create_attendance_record("clock_in")
}

pub fn clock_out() -> Result<exit_codes::ExitCode> {
    create_attendance_record("clock_out")
}

pub fn start_break() -> Result<exit_codes::ExitCode> {
    create_attendance_record("start_break")
}

pub fn end_break() -> Result<exit_codes::ExitCode> {
    create_attendance_record("end_break")
}
