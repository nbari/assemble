use std::error::Error;
use std::io::Write;
use std::str::FromStr;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// # Errors
///
/// Will return `Err` if `color` does not exist or canot reset stdout
pub fn print(msg: &str, color: &str) -> Result<(), Box<dyn Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::from_str(color)?)))?;
    writeln!(&mut stdout, "{}", msg)?;
    Ok(stdout.reset()?)
}
