mod commands;
mod utils;

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
            "generate" | "g" | "create" => {
                let abstraction = std::env::args()
                    .nth(3)
                    .expect("You should provide an ember abstraction");
                let name = std::env::args().nth(4).unwrap();

                return commands::generate::run(abstraction, name);
            }
            "delete" | "d" | "destroy" => {
                let abstraction = std::env::args()
                    .nth(3)
                    .expect("You should provide an ember abstraction");
                let name = std::env::args().nth(4).unwrap();

                return commands::destroy::run(abstraction, name);
            }
            _ => {
                println!("{}", Paint::red("unknown command. Available options are:"));

                commands::help::run()?;

                return Err(Error::new(ErrorKind::Other, "Exiting with error"));
            }
        },
    }
}
