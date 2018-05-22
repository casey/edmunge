extern crate executable_path;
extern crate tempfile;

use executable_path::executable_path;
use tempfile::Builder;
use std::{
  fs,
  process::Command,
};

macro_rules! test {
  (
    name:   $name:ident,
    script: $script:expr,
    input:  $input:expr,
    output: $output:expr,
  ) => {
    #[test]
    fn $name() {
      let tempdir = Builder::new()
        .prefix(concat!("edmunge-test-", stringify!($name)))
        .tempdir()
        .expect("Failed to create temporary directory.");

      let script = String::from("#!/usr/bin/env edmunge\n") + $script;

      let script_path = tempdir.path().join("script");

      fs::write(&script_path, script)
        .expect("Failed to write script.");

      let input = $input;

      let document_path_a = tempdir.path().join("document-a");

      fs::write(&document_path_a, input)
        .expect("Failed to write document.");

      let document_path_b = tempdir.path().join("document-b");

      fs::write(&document_path_b, input)
        .expect("Failed to write document.");

      let output = Command::new(&executable_path("edmunge"))
        .arg(&script_path)
        .arg(&document_path_a)
        .arg(&document_path_b)
        .output()
        .expect("Failed to launch edmunge.");

      let actual_a = fs::read_to_string(&document_path_a)
        .expect("Failed to read document a.");

      let actual_b = fs::read_to_string(&document_path_a)
        .expect("Failed to read document a.");

      assert_eq!(actual_a, actual_b);

      let expected = $output;

      if actual_a != expected {
        eprintln!("{:?}", output);
        assert_eq!(actual_a, expected);
      }
    }
  };
}

test! {
  name:   simple,
  script: ",s/a/b/g\nw",
  input:  "a\n",
  output: "b\n",
}

