use assemble::{color, command, config};
use clap::{App, Arg};
use compound_duration::format_dhms;
use std::io::{self, Write};
use std::process;
use std::time::Instant;

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

    let now = Instant::now();
    for (i, k) in (&yml.steps).iter().enumerate() {
        let start = Instant::now();
        if let Err(e) = color::print(format!("STEP {} [{}]", i + 1, k.name).as_str(), "yellow") {
            eprintln!("{:?}", e);
        }
        match command::run(&k.cmd, &yml.env) {
            Ok(output) => {
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
                if !output.status.success() {
                    if let Err(e) = color::print(
                        format!("Error in step {} [{}]", i + 1, k.name).as_str(),
                        "red",
                    ) {
                        eprintln!("{:?}", e);
                    }
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("error executing command: {}", e);
                process::exit(1);
            }
        }

        if let Err(e) = color::print(
            format!(
                "ok [{}] in {}",
                k.name,
                format_dhms(start.elapsed().as_secs() as usize)
            )
            .as_str(),
            "green",
        ) {
            eprintln!("{:?}", e);
        }
        println!();
    }

    // print finished
    if let Err(e) = color::print(
        format!(
            "Finished in {}",
            format_dhms(now.elapsed().as_secs() as usize)
        )
        .as_str(),
        "green",
    ) {
        eprintln!("{:?}", e);
    }
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
