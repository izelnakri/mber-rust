// TODO: get application_name from project.json
// TODO: singularize/format the input_string
use super::super::utils;
use super::super::utils::console;
use std::process;

use super::super::generators::{component, helper, initializer, instance_initializer, mixin, model, route, service, util};

pub fn run() -> std::io::Result<()> {
    let abstraction = std::env::args()
        .nth(2)
        .unwrap_or_else(|| {
            console::error("mber g missing an ember abstraction to generate!");

            process::exit(1);
        });
    let ember_abstractions = vec![
        "component", "helper", "initializer", "instance-initializer", "mixin", "model", "route", "service", "util"
    ];

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
    let project_root = utils::find_project_root();
    // TODO: get application_name from project.json

    match abstraction.as_str() {
        "component" => component::generate(name, "something", project_root)?,
        "helper" => helper::generate(name, "something", project_root)?,
        "initializer" => initializer::generate(name, "something", project_root)?,
        "instance_initializer" => instance_initializer::generate(name, "something", project_root)?,
        "mixin" => mixin::generate(name, "something", project_root)?,
        "model" => model::generate(name, "something", project_root)?,
        "route" => route::generate(name, "something", project_root)?,
        "service" => service::generate(name, "something", project_root)?,
        "util" => util::generate(name, "something", project_root)?,
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
