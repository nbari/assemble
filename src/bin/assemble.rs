use assemble::{color, command, config};
use clap::{App, Arg};
use compound_duration::{format_dhms, format_ns};
use std::process;
use std::time::Instant;

fn main() {
    let matches = App::new("assemble")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("config")
                .help("asm.yml")
                .long("config")
                .default_value("asm.yml")
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

    if yml.build.is_none() && yml.deploy.is_none() {
        eprintln!("Need to define: 'build', 'deploy' or both.");
        process::exit(1);
    }

    let now = Instant::now();
    if let Some(build) = &yml.build {
        for (i, k) in build.iter().enumerate() {
            let start = Instant::now();
            color::print(format!("STEP {} [{}]", i + 1, k.name).as_str(), "yellow");
            println!();
            if let Some(cmd) = &k.r#do {
                match command::run(cmd, &yml.env) {
                    Ok(mut child) => match child.wait() {
                        Ok(status) => {
                            if !status.success() {
                                println!();
                                color::print(
                                    format!("Error in step {} [{}]", i + 1, k.name).as_str(),
                                    "red",
                                );
                                process::exit(1);
                            }
                        }
                        Err(e) => {
                            eprintln!("error attempting to wait: {}", e);
                            process::exit(1);
                        }
                    },
                    Err(e) => {
                        eprintln!("error executing command: {}", e);
                        process::exit(1);
                    }
                }
                color::print(format!("ok [{}]", k.name,).as_str(), "green");
                println!(" in {}", format_ns(start.elapsed().as_nanos() as usize));
                println!();
            }

            // PUT
            if let Some(put) = &k.put {
                println!("upload a file {}", put);
            }
        }
    }

    // print finished
    color::print(
        format!(
            "Assemble finished in {}",
            format_dhms(now.elapsed().as_secs() as usize)
        )
        .as_str(),
        "green",
    );
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
