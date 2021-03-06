use assemble::{archive, color, command, config, git};
use chrono::prelude::{SecondsFormat, Utc};
use clap::{App, Arg};
use compound_duration::{format_dhms, format_ns};
use std::collections::BTreeMap;
use std::process;
use std::sync::Arc;
use std::time::Instant;
use std::{thread, time};
use tempfile::TempDir;

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

    // get git latest_commit
    let version = match git::latest_commit() {
        Ok(v) => v,
        _ => Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
    };

    // find way to only use this if PUT is used
    let tmp_dir = match TempDir::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error creating tmpdir: {}", e);
            process::exit(1);
        }
    };

    println!("{:#?}", &yml.storage);

    /*
     need S3M to continue
    */

    // time the tasks
    let now = Instant::now();
    if let Some(build) = &yml.build {
        for (i, k) in build.iter().enumerate() {
            let start = Instant::now();
            match k {
                config::Build::Make(s) => {
                    step_make(i, s, s, &yml.env);
                    println!(" in {}", format_ns(start.elapsed().as_nanos()));
                    println!();
                }
                // steps (associative array)
                config::Build::Step(s) => {
                    // DO
                    if let Some(cmd) = &s.make {
                        step_make(i, cmd, &s.name, &yml.env);
                        println!(" in {}", format_ns(start.elapsed().as_nanos()));
                        println!();
                    }
                    // PUT
                    if let Some(file) = &s.put {
                        color::print(format!("STEP {} [{}]", i + 1, s.name).as_str(), "yellow");
                        println!();
                        let packet = match archive::pack(&file, tmp_dir.path()) {
                            Ok(p) => println!("{:?}, {}", p.file, p.checksum),
                            Err(e) => {
                                color::print(format!("Error archiving {}: ", file).as_str(), "red");
                                println!("{}", e);
                                process::exit(1);
                            }
                        };
                        color::print(format!("ok [{}]", s.name,).as_str(), "green");
                        println!(" in {}", format_ns(start.elapsed().as_nanos()));
                        println!();
                    }
                }
            }
        }

        // print finished
        color::print(
            format!(
                "Assemble finished [{}] in {}\nversion: ",
                &yml.name,
                format_dhms(now.elapsed().as_secs() as usize),
            )
            .as_str(),
            "green",
        );
        println!("{}", version);
    }

    let delay = time::Duration::from_secs(3);
    loop {
        println!("sleeping for 3  sec ");
        thread::sleep(delay);
    }
}

// step DO (run a command)
fn step_make(i: usize, cmd: &str, name: &str, env: &BTreeMap<String, String>) {
    color::print(format!("STEP {} [{}]", i + 1, name).as_str(), "yellow");
    println!();
    match command::run(cmd, env) {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if !status.success() {
                    println!();
                    color::print(
                        format!("Error in step {} [{}]", i + 1, name).as_str(),
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
    color::print(format!("ok [{}]", name,).as_str(), "green");
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
