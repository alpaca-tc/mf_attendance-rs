use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn build_api_token_path() -> String {
    let mut s = String::new();

    if let Ok(root) = env::var("HOME") {
        s.push_str(root.as_ref());
    }

    s.push_str("/.config/mf_attendance");
    s
}

fn remove_api_token() {
    let path_string = build_api_token_path();
    let path = Path::new(&path_string);

    if path.exists() {
        fs::remove_file(path).unwrap();
    }
}

pub fn api_token_exists() -> bool {
    let path_string = build_api_token_path();
    let path = Path::new(&path_string);
    path.exists()
}

pub fn create_api_token(api_token: &[u8]) -> Result<bool> {
    if api_token_exists() {
        remove_api_token();
    }

    let path_string = build_api_token_path();
    let path = Path::new(&path_string);

    if let Some(directory) = path.parent() {
        if !directory.exists() {
            fs::create_dir(directory)?;
        }
    }

    let mut file = fs::File::create(path)?;
    file.write_all(api_token)?;

    // 何を返すべき？
    Ok(true)
}

pub fn fetch_api_token_from_file() -> Result<String> {
    let path_string = build_api_token_path();
    let path = Path::new(&path_string);

    if path.exists() {
        let mut file = File::open(&path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(buffer)
    } else {
        Err(anyhow!("file not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_token_exists_returns_true_when_api_token_doesnt_exists() {
        remove_api_token();
        assert_eq!(api_token_exists(), false)
    }

    #[test]
    fn api_token_exists_returns_true_when_api_token_exists() {
        remove_api_token();
        create_api_token("xxx".as_bytes());
        assert!(api_token_exists());
    }

    #[test]
    fn fetch_api_token_from_file_when_api_token_is_not_found() {
        remove_api_token();

        let result = fetch_api_token_from_file();
        assert!(result.is_err());
    }
}
