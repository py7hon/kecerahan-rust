use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process;

use docopt::Docopt;
use serde::Deserialize;
use glob::glob;

const USAGE: &'static str = "
kecerahan - Control screen brightness from the command line.

Usage:
  kecerahan [options] (-b <brightness_value> | --brightness <brightness_value>)
  kecerahan -h | --help

Options:
  -b, --brightness  Set the brightness level (1-999).
  -h, --help        Show this help message and exit.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_brightness: Option<String>,
    flag_h: bool,
    flag_help: bool,
}

fn main() -> io::Result<()> {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("This program requires root privileges. Please run it with sudo.");
        process::exit(1);
    }

    let _args: Vec<String> = env::args().collect();

    let docopt: Args = match Docopt::new(USAGE).and_then(|d| d.deserialize()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    if docopt.flag_h || docopt.flag_help {
        println!("{}", USAGE);
        process::exit(0);
    }

    let brightness_value = docopt.flag_brightness.unwrap_or_else(|| {
        eprintln!("Error: Brightness value is required.");
        process::exit(1);
    });

    if let Some(device_path) = glob("/sys/class/backlight/*/brightness")
        .expect("Failed to glob devices")
        .filter_map(Result::ok)
        .next()
        .map(|path| path.to_string_lossy().into_owned())
    {
        let mut device_file = File::create(device_path)?;

        device_file.write_all(brightness_value.as_bytes())?;

        println!("Brightness set successfully.");
        Ok(())
    } else {
        eprintln!("No backlight devices found.");
        process::exit(1);
    }
}
