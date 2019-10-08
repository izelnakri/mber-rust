use super::super::types::cli_arguments::CLIArguments;
use super::super::utils;
use super::super::utils::{console};

pub fn run() -> std::io::Result<()> {
    console::log("Building the application...");

    let project_root = utils::find_project_root();
    let cli_arguments = CLIArguments::new();

    println!("cli_arguments are {:?}", cli_arguments);

    Ok(())
}
