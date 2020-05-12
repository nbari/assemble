use std::io::Write;
use std::str::FromStr;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// # Errors
///
/// Will return `Err` if `color` does not exist or canot reset stdout
pub fn print(msg: &str, color: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if let Ok(c) = Color::from_str(color) {
        if stdout.set_color(ColorSpec::new().set_fg(Some(c))).is_ok() {
            if write!(&mut stdout, "{}", msg).is_err() {
                print!("{}", msg);
            }
            if let Err(e) = stdout.reset() {
                println!("there was a problem resetting the color settings: {}", e);
            }
        }
    }
}
