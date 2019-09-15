// cargo:rerun-if-changed=build.rs
use core::str::Split;
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Display;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum KeyValue {
    String(String),
    RefCell(HashMap<String, KeyValue>),
}

fn main() {
    // TODO: inject version to the help

    // TODO: create { file to filename } to src/utils/new-ember-app-boilerplate-json.rb
    let walker = WalkDir::new("ember-app-boilerplate").into_iter();
    let mut file_system_map: RefCell<HashMap<String, KeyValue>> = RefCell::new(HashMap::new());

    for entry in walker {
        let entry = entry.unwrap();

        match entry.file_type().is_dir() {
            true => {
                check_and_set_directory_map_to_map(&mut file_system_map, entry.path().display())
            }
            false => {
                find_directory_map_and_insert_file(&mut file_system_map, entry.path().display())
            }
        }
    }

    let json_string = serde_json::to_string(&file_system_map).unwrap();

    fs::write(
        "src/utils/ember_app_boilerplate.rs",
        format!(
            "pub fn as_string() -> &'static str {{\nreturn r##\"\n{}\"##;\n}}",
            &json_string
        ),
    )
    .expect("couldnt write to src/utils/ember_app_boilerplate.rs");
}

fn check_and_set_directory_map_to_map(
    file_system_map: &mut RefCell<HashMap<String, KeyValue>>,
    path: Display,
) {
    let full_path = path.to_string();
    let directories = full_path.split("/");

    for (index, directory) in directories.clone().enumerate() {
        let mut mutable_file_system_map = file_system_map.borrow_mut();

        if let None = mutable_file_system_map.get(&String::from(directory)) {
            let target_hash_map =
                get_from_directory_map_from_map(&mut mutable_file_system_map, &directories, index);
            target_hash_map
                .entry(String::from(directory))
                .or_insert(KeyValue::RefCell(HashMap::new()));
        };
    }
}

fn get_from_directory_map_from_map<'a>(
    hashmap: &'a mut HashMap<String, KeyValue>,
    directories_list: &Split<&str>,
    directory_index: usize,
) -> &'a mut HashMap<String, KeyValue> {
    let directory_list: Vec<&str> = directories_list.clone().collect::<Vec<_>>(); // NOTE: this is correct but optimize it!
    let target_directory_list = directory_list.get(0..directory_index).unwrap();

    // TODO: move this to a private util function:
    return target_directory_list.iter().enumerate().fold(
        hashmap,
        |acc: &mut HashMap<String, KeyValue>,
         (_index, directory_name)|
         -> &mut HashMap<String, KeyValue> {
            match acc.get_mut(*directory_name).unwrap() {
                KeyValue::RefCell(something) => something,
                _ => panic!(
                    "{} must exist in the fs directory mapping in memory!",
                    directory_name
                ),
            }
        },
    );
}

// TODO: currently cant embed non-UTF-8 to json(png files etc)
fn find_directory_map_and_insert_file(
    file_system_map: &mut RefCell<HashMap<String, KeyValue>>,
    path: Display,
) {
    let path_string = path.to_string();
    let path_list = &path_string.split("/");
    let file_name = &path_string.split("/").last().unwrap();
    let contents =
        fs::read_to_string(&path_string).expect(format!("couldnt read {}", path_string).as_str());
    let mut full_map = file_system_map.borrow_mut();
    let directory_path_index = &path_list.clone().count().wrapping_sub(1);
    let target_hash_map =
        get_from_directory_map_from_map(&mut full_map, &path_list, *directory_path_index);

    target_hash_map.insert(file_name.to_string(), KeyValue::String(contents));
}
