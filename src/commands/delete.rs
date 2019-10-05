use super::super::utils::console;
use std::process;

pub fn run() -> std::io::Result<()> {
    let abstraction = std::env::args()
        .nth(2)
        .unwrap_or_else(|| {
            console::error("mber d missing an ember abstraction to delete!");

            process::exit(1);
        });
    let ember_abstractions = vec!["component", "helper", "initializer", "instance-initializer", "mixin", "model", "route", "service", "util"];

    if !ember_abstractions.contains(&abstraction.as_str()) {
        console::log(format!("{} is not a valid ember abstraction to delete. Choose one of these abstractions:", abstraction));
        println!("{:?}", ember_abstractions);

        process::exit(1);
    }

    let _name = std::env::args().nth(3)
        .unwrap_or_else(|| {
            console::error(format!("mber d {} missing a name to delete!", abstraction));

            process::exit(1);
        });


    Ok(())

    // return commands::destroy::run(abstraction, name);
}
