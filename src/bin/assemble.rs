use assemble::config;
use clap::{App, Arg};
use serde_yaml;
use std::process;

fn main() {
    let matches = App::new("assemble")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("config")
                .help("asm.yml")
                .long("config")
                .short("c")
                .required(true)
                .value_name("FILE")
                .validator(is_file),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap_or_else(|| {
        eprintln!("Unable to open configuration file, use (\"-h for help\")");
        process::exit(1);
    });

    // parse config file
    let file = std::fs::File::open(&config).expect("Unable to open file");
    let yml: config::Config = match serde_yaml::from_reader(file) {
        Err(e) => {
            eprintln!("Error parsing configuration file: {}", e);
            process::exit(1);
        }
        Ok(yml) => yml,
    };

    println!("{:?}", yml);
}

fn is_file(s: String) -> Result<(), String> {
    let metadata = match std::fs::metadata(&s) {
        Err(err) => return Err(err.to_string()),
        Ok(metadata) => metadata,
    };
    if !metadata.is_file() {
        return Err(format!("cannot read file: {}", s));
    }
    Ok(())
}
