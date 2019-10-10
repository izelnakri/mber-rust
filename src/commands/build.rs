use std::process::Command;
// use super::super::types::cli_arguments::CLIArguments;
use super::super::utils;
use super::super::utils::{console};

pub fn run() -> std::io::Result<()> {
    console::log("Building the application...");

    let project_root = utils::find_project_root();
    let project_root_path = project_root.to_string_lossy();
    let command = Command::new("node")
        .args(&["-e", format!("
            require('{}/index.js')({{ }});
        ", project_root_path).as_str()])
        .current_dir(project_root)
        .output()
        .expect("couldnt run node index.js on the project");

    let stdout = String::from_utf8(command.stdout.to_vec()).unwrap();
    let stderr = String::from_utf8(command.stderr.to_vec()).unwrap();
    println!("{}", stdout);
    println!("stderr: {}", stderr);

    // let cli_arguments = CLIArguments::new();
    // TODO: get ENV

    // println!("cli_arguments are {:?}", cli_arguments);

    // TODO: run {project_root}/index.js)(ENV) and then.. it returns buildConfig
    // cast buildConfig JS Value to my rust types
    // use that buildConfig to buildDistFolder(which triggers build functions and more)

    Ok(())
}
