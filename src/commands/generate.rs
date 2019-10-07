use super::super::utils::console;
use std::process;

use super::super::generators::{component};

pub fn run() -> std::io::Result<()> {
    let abstraction = std::env::args()
        .nth(2)
        .unwrap_or_else(|| {
            console::error("mber g missing an ember abstraction to generate!");

            process::exit(1);
        });
    let ember_abstractions = vec!["component", "helper", "initializer", "instance-initializer", "mixin", "model", "route", "service", "util"];

    if !ember_abstractions.contains(&abstraction.as_str()) {
        console::error(format!("{} is not a valid ember abstraction to generate. Choose one of these abstractions:", abstraction));
        println!("{:?}", ember_abstractions);

        process::exit(1);
    }

    let name = std::env::args().nth(3)
        .unwrap_or_else(|| {
            console::error(format!("mber g {} missing a name to generate!", abstraction));

            process::exit(1);
        });

    match abstraction.as_str() {
        "component" => component::generate(name, "something")?,
    //     "helper" => helper::generate(name),
    //     "initializer" => initializer::generate(name),
    //     "instance_initializer" => instance_initializer::generate(name),
    //     "mixin" => mixin::generate(name),
    //     "model" => model::generate(name),
    //     "route" => route::generate(name),
    //     "service" => service::generate(name),
    //     "util" => util::generate(name),
        _ => ()
    }

    Ok(())
}

// extern crate tokio;

// use tokio::prelude::{AsyncRead, Future};

// let task = tokio::fs::File::open("./Cargo.toml")
//     .and_then(|mut file| {
//         let mut contents = vec![];

//         file.read_buf(&mut contents)
//             .map(|_res| println!("{}", String::from_utf8(contents).unwrap()))
//     }).map_err(|err| eprintln!("IO error: {:?}", err));

// let task = tokio::fs::File::open("./Cargo.toml")
//     .and_then(|mut file| {
//         // do something with the file ...
//         let string = String::new();

//         // file.read_to_string(&mut string);
//         // println!("{}", string);
//         file
//     })
//     .map_err(|e| {
//         // handle errors
//         eprintln!("IO error: {:?}", e);
//     });

// tokio::run(task);
// oo
