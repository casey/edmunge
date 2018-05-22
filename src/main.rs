extern crate clap;
extern crate regex;

use std::{
  fs,
  io::Write,
  process::{self, Command, Stdio},
};

use regex::Regex;
use clap::{App, Arg, AppSettings};

pub fn main() {
  let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(concat!("v", env!("CARGO_PKG_VERSION")))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(concat!(env!("CARGO_PKG_DESCRIPTION"), " - ", env!("CARGO_PKG_HOMEPAGE")))
    .help_message("Print help information")
    .version_message("Print version information")
    .setting(AppSettings::ColoredHelp)
    .arg(Arg::with_name("SCRIPT")
         .takes_value(true)
         .required(true)
         .help("Execute commands from SCRIPT"))
    .arg(Arg::with_name("FILES")
         .takes_value(true)
         .required(true)
         .multiple(true)
         .help("Files to edit"))
    .get_matches();

  let script_path = matches.value_of("SCRIPT")
    .expect("SCRIPT argument value missing");

  let files = matches.values_of("FILES")
    .expect("FILES argument value missing");

  let script = fs::read_to_string(script_path)
    .expect("Failed to read SCRIPT");

  let preprocessed = preprocess(&script);

  for file in files {
    let mut ed = Command::new("ed")
      .arg(file)
      .stdin(Stdio::piped())
      .spawn()
      .expect("Failed to launch ed.");

    ed.stdin.as_mut()
      .expect("ed process had no stdin.")
      .write(preprocessed.as_bytes())
      .expect("Failed to write script to ed process stdin.");

    let status = ed.wait()
      .expect("Failed to wait for ed.");

    match status.code() {
      Some(0) => {}
      Some(code) => {
        eprintln!("ed failed with code {}", code);
        process::exit(code);
      }
      None => {
        eprintln!("ed terminated by signal");
        process::exit(-1);
      }
    }
  }
}

fn preprocess(script: &str) -> String {
  let regex = Regex::new("^#!.*\n")
    .expect("Failed to compile shebang regex.");

  // remove a leading shebang
  let output = regex.replace(&script, "");

  // prepend an H to turn on verbose error messages
  let mut output = String::from("H\n") + &output;

  // make sure there's a trailing newline
  match output.pop() {
    None | Some('\n') => output.push('\n'),
    Some(c) => {
      output.push(c);
      output.push('\n');
    }
  }

  output
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn preprocessor() {
    let input  = "#!/usr/bin/env edmunge\nfoo";
    let actual = preprocess(input);
    assert_eq!(actual, "H\nfoo\n");
    let input  = "#!/usr/bin/env edmunge\n#!/usr/bin/env edmunge\nfoo";
    let actual = preprocess(input);
    assert_eq!(actual, "H\n#!/usr/bin/env edmunge\nfoo\n");
  }
}
