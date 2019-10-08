mod commands;
pub mod types;
pub mod utils;
mod generators;

use std::io::{Error, ErrorKind, Result};
use yansi::Paint;

fn main() -> Result<()> {
    if let Ok(true) = std::env::var("FORCE_COLOR").map(|v| v == "0") {
        Paint::disable();
    }

    match std::env::args().nth(1) {
        None => commands::help::run(),
        Some(command) => match command.as_str() {
            "server" | "serve" | "s" => commands::server::run(),
            "test" | "t" => commands::test::run(),
            "build" | "b" => commands::build::run(),
            "console" | "c" => commands::console::run(),
            "help" | "h" => commands::help::run(),
            "init" | "new" => commands::new::run(),
            "generate" | "g" | "create" => commands::generate::run(),
            "delete" | "d" | "destroy" => commands::delete::run(),
            _ => {
                println!("{}", Paint::red("unknown command. Available options are:"));

                commands::help::run()?;

                return Err(Error::new(ErrorKind::Other, "Exiting with error"));
            }
        },
    }
}
