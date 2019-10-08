use std::env;
use std::fs::File;
use std::process;
use super::super::utils;
use super::super::utils::console;
use super::super::generators::{component, helper, initializer, instance_initializer, mixin, model, route, service, util};
use serde_json;
use serde_json::value::Value;

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

    let remaining_args = env::args().skip(3).collect::<Vec<_>>();

    if remaining_args.len() == 0 {
        console::error(format!("mber g {} missing a name to generate!", abstraction));

        process::exit(1);
    }

    let name = remaining_args.join(" ");
    let project_root = utils::find_project_root();
    let package_json: Value = serde_json::from_reader(
        File::open(format!("{}/package.json", project_root.to_string_lossy()))?
    )?;
    let application_name = package_json["name"].as_str().unwrap();

    match abstraction.as_str() {
        "component" => component::generate(name, application_name, project_root)?,
        "helper" => helper::generate(name, application_name, project_root)?,
        "initializer" => initializer::generate(name, application_name, project_root)?,
        "instance_initializer" => instance_initializer::generate(name, application_name, project_root)?,
        "mixin" => mixin::generate(name, application_name, project_root)?,
        "model" => model::generate(name, application_name, project_root)?,
        "route" => route::generate(name, application_name, project_root)?,
        "service" => service::generate(name, application_name, project_root)?,
        "util" => util::generate(name, application_name, project_root)?,
        _ => ()
    }

    Ok(())
}
