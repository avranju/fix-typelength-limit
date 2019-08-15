use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{self, Command, Output};
use std::str;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref RE: Regex = Regex::new(r###"type_length_limit *= *"([0-9]+)""###).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: fix-typelength-limit <program> [<args>]");
        process::exit(1);
    }

    // run the build command
    loop {
        let output = build(&args).expect("Failed to run build");
        if !output.status.success() {
            // look for the type-length limit fix in the error message
            let err = str::from_utf8(&output.stderr)
                .expect("Could not read error message as utf-8 string");
            log(&format!("Build failed with:\n{}", err));

            if let Some(new_limit) = RE
                .captures(err)
                .and_then(|caps| caps.get(1))
                .map(|m| m.as_str())
            {
                fix_limit(new_limit).expect("Replacing code failed");
                log("Fixed typed length limit error. Retrying build.");
            } else {
                log("Build error was not type length limit error.");
                break;
            }
        } else {
            log("Build was successful.");
            break;
        }
    }
}

fn log(msg: &str) {
    println!(">>> {}", msg);
}

fn build(args: &Vec<String>) -> Result<Output, io::Error> {
    log(&format!("Running: {}", args.join(" ")));
    Command::new(&args[0]).args(&args[1..]).output()
}

fn fix_limit(new_limit: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    // use `lib.rs` if it exists or fallback to `main.rs`
    let file_name = if Path::new("src/lib.rs").exists() {
        "src/lib.rs"
    } else {
        "src/main.rs"
    };

    let data = fs::read(file_name)?;
    let src = String::from_utf8_lossy(&data);
    let src = RE.replace(&src, |_: &Captures| {
        format!("type_length_limit = \"{}\"", new_limit)
    });

    fs::write(file_name, src.as_bytes())?;
    Ok(())
}
