use std::collections::BTreeMap;
use std::error::Error;
use std::process::{Command, Output};

/// # Errors
///
/// Will return `Err` if can't execute the command
/// permission to read it.
pub fn run(cmd: &str, env: &BTreeMap<String, String>) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .env_clear()
        .envs(env)
        .output()?)
}
