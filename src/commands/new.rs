// TODO: rewrite this in tokio
use super::super::utils::console;
use super::super::utils::ember_app_boilerplate;
use mustache::MapBuilder;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use yansi::Paint;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum KeyValue {
    String(String),
    RefCell(HashMap<String, KeyValue>),
}

pub fn run() -> std::io::Result<()> {
    let application_name = std::env::args().nth(2).unwrap_or_else(|| {
        println!("You forgot to include an application name! Example: mber init example-app");

        process::exit(1);
    });

    let path = env::current_dir()?;
    let user_has_the_app_in_path: bool = path
        .iter()
        .any(|folder| folder.to_str().unwrap() == application_name);

    if user_has_the_app_in_path || user_has_app_in_current_directory(&path, &application_name) {
        console::log(format!("{} already exists!", application_name));

        process::exit(1);
    }

    console::log(format!("creating {} application", application_name));

    let current_directory = path.display().to_string();
    let application_directory = format!("{}/{}", &current_directory, &application_name);
    let fs_hashmap: HashMap<String, KeyValue> =
        serde_json::from_str(ember_app_boilerplate::as_string()).unwrap();

    create_nested_directory_and_files_from_hashmap(
        &fs_hashmap,
        current_directory,
        &application_name,
    );

    add_application_name_to_boilerplate_files(application_directory, &application_name)?;

    if let KeyValue::RefCell(hashmap) = fs_hashmap.get("ember-app-boilerplate").unwrap() {
        for (key, _value) in hashmap {
            println!("{} {}", Paint::green("created"), key);
        }
    }

    console::log(Paint::green(format!(
        "{} ember application created. Next is to do:",
        application_name
    )));
    println!("$ cd {} && npm install && mber s", application_name);

    Ok(())
}

fn user_has_app_in_current_directory(path: &PathBuf, application_name: &str) -> bool {
    return fs::read_dir(path)
        .unwrap()
        .any(|element| element.unwrap().file_name() == application_name);
}

fn create_nested_directory_and_files_from_hashmap(
    fs_hashmap: &HashMap<String, KeyValue>,
    current_directory: String,
    application_name: &String,
) {
    for (file_or_directory_name, value) in fs_hashmap {
        let target_path = if &file_or_directory_name.as_str() == &"ember-app-boilerplate" {
            format!("{}/{}", &current_directory, &application_name).to_string()
        } else {
            format!("{}/{}", &current_directory, &file_or_directory_name).to_string()
        };

        match value {
            KeyValue::String(content) => {
                fs::write(&target_path, content)
                    .expect(format!("couldnt write to {}", target_path).as_str());
                ();
            }
            KeyValue::RefCell(directory_map) => {
                fs::create_dir(&target_path)
                    .expect(format!("couldnt create dir! {}", target_path).as_str());
                create_nested_directory_and_files_from_hashmap(
                    &directory_map,
                    target_path,
                    application_name,
                )
            }
        }
    }

    return ();
}

fn add_application_name_to_boilerplate_files(
    application_directory: String,
    application_name: &String,
) -> std::io::Result<()> {
    let application_name = application_name.as_str();
    add_application_data_to_file(
        format!("{}/config/environment.js", &application_directory),
        &application_name,
    )
    .unwrap();
    add_application_data_to_file(
        format!("{}/package.json", &application_directory),
        &application_name,
    )
    .unwrap();
    add_application_data_to_file(
        format!("{}/tests/index.html", &application_directory),
        &application_name,
    )
    .unwrap();

    fs::write(
        format!("{}/.gitinote", &application_directory),
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            ".cache", "dist", "node_modules", "npm-debug.log*", "yarn-error.log", "tmp"
        ),
    )?;

    return Ok(());
}

fn add_application_data_to_file(
    file_path: String,
    application_name: &str,
) -> Result<(), std::io::Error> {
    let application_data = MapBuilder::new()
        .insert_str("applicationName", application_name)
        .build();
    let file_template = mustache::compile_path(&file_path).unwrap();

    return fs::write(
        file_path,
        file_template
            .render_data_to_string(&application_data)
            .unwrap(),
    );
}
