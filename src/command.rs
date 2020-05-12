use std::collections::BTreeMap;
use std::error::Error;
use std::process::{Child, Command};

/// # Errors
///
/// Will return `Err` if can't execute the command
/// permission to read it.
pub fn run(cmd: &str, env: &BTreeMap<String, String>) -> Result<Child, Box<dyn Error>> {
    if env.is_empty() {
        Ok(Command::new("sh").arg("-c").arg(cmd).spawn()?)
    } else {
        Ok(Command::new("sh").arg("-c").arg(cmd).envs(env).spawn()?)
    }
}
