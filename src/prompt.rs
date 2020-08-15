use anyhow::Result;
use std::io::{BufRead, BufReader, Write};

pub fn ask(message: &str) -> Result<String> {
    write!(&mut std::io::stdout(), "{}", message)?;
    std::io::stdout().flush()?;

    let mut s = String::new();
    let mut stdin = std::io::stdin();
    let mut input = BufReader::new(&mut stdin);
    input.read_line(&mut s)?;
    Ok(String::from(s.trim()))
}

pub fn echo(message: &str) -> Result<()> {
    write!(&mut std::io::stdout(), "{}", message)?;
    std::io::stdout().flush()?;

    Ok(())
}
