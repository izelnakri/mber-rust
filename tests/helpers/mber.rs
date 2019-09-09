use std::process::{Command, Output};

pub fn spawn(command_string: &str) -> (String, Output) {
    let mut command = Command::new("cargo");

    command.env("FORCE_COLOR", "0").args(&["run"]);

    if command_string != "" {
        let arguments: Vec<&str> = command_string.split(" ").collect();

        command.args(&arguments);
    }

    let output = command.output().expect("Failed to execute the process");

    return (String::from_utf8(output.stdout.to_vec()).unwrap(), output);
}

// use std::path::Path;
// let path = Path::new("fixtures/uri");
// let result = load_file(&path);
//
