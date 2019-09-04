mod commands;

use yansi::Paint;

fn main() {
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
            "delete" | "d" | "destroy" => commands::destroy::run(),
            _ => {
                println!("{}", Paint::red("unknown command. Available options are:"));
                commands::help::run();
                std::process::exit(1);
            }
        },
    }
}
