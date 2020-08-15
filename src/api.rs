use crate::api_token;
use anyhow::Result;
use std::collections::HashMap;

fn post(path: &str, params: HashMap<&str, &str>) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::new();
    let api_token = api_token::fetch_api_token_from_file()?;

    let mut token = String::from("Token ");
    token.push_str(&*api_token);

    let response = client
        .post(path)
        .header("Authorization", token)
        .json(&params)
        .send()?;

    Ok(response)
}

pub fn create_attendance_record(event: &str, time: &str) -> Result<reqwest::blocking::Response> {
    let mut map: HashMap<&str, &str> = HashMap::new();

    map.insert("event", event);
    map.insert("user_time", time);

    let response = post(
        "https://attendance.moneyforward.com/api/external/beta_feature/me/attendance_records",
        map,
    )?;

    Ok(response)
}
