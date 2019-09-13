// cargo:rerun-if-changed=build.rs
// extern crate serde;
// extern crate serde_json;

use core::str::Split;
// use serde::ser::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Display;
use walkdir::WalkDir;

#[derive(Debug)]
// #[serde(untagged)]
enum KeyValue {
    String(String),
    RefCell(HashMap<String, KeyValue>),
    None,
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

    // let serialized = serde_json::to_string(&file_system_map.into_inner()).unwrap();
    // println!("serialized = {}", serialized);

    for (key, value) in file_system_map.into_inner().into_iter() {
        println!("{}: {:?}", key, value);
    }
}

fn check_and_set_directory_map_to_map(
    mut file_system_map: &mut RefCell<HashMap<String, KeyValue>>,
    path: Display,
) {
    let full_path = path.to_string();
    let directories = full_path.split("/");

    for (index, directory) in directories.clone().enumerate() {
        let mut mutable_file_system_map = file_system_map.borrow_mut();

        match mutable_file_system_map.get(&String::from(directory)) {
            None => {
                // println!("{:?}", directory);
                if directory == "ember-app-boilerplate" {
                    mutable_file_system_map.insert(
                        String::from("ember-app-boilerplate"),
                        KeyValue::RefCell(HashMap::new()),
                    );
                    return;
                }
                // let mut mutable_file_system_map = file_system_map.borrow_mut();
                let mut target_hash_map = get_from_directory_map_from_map(
                    &mut mutable_file_system_map,
                    &directories,
                    index,
                );

                target_hash_map.insert(String::from(directory), KeyValue::RefCell(HashMap::new()))
            }
            Some(_) => None,
        };
    }
}

fn get_from_directory_map_from_map<'a>(
    mut hashmap: &'a mut HashMap<String, KeyValue>,
    directories_list: &Split<&str>,
    directory_index: usize,
) -> &'a mut HashMap<String, KeyValue> {
    let directory_list: Vec<&str> = directories_list.clone().collect::<Vec<_>>(); // NOTE: this is correct but optimize it!

    println!("list count is {}", directory_list.len());

    if ((directory_index) == directory_list.len()) {
        // println!("RETURNS DIRECTLY FOR {:?} | {}", directory_list, directory_index);
        // let mut reference = &hashmap;

        return hashmap;
    }

    // println!("BRANCH CALL FOR {:?}", directory_list);

    println!("index is {}", directory_index);
    println!("directory_list is {:?}", directory_list);
    let target_directory_list = &directories_list.clone().take(directory_index);
    println!(
        "target_directory_list is {:?}",
        target_directory_list
            .clone()
            .enumerate()
            .collect::<Vec<_>>()
    );

    // TODO: move this to a private util function:
    let result = target_directory_list.clone().enumerate().fold(
        hashmap,
        |mut acc: &mut HashMap<String, KeyValue>,
         (index, directory_name)|
         -> &mut HashMap<String, KeyValue> {
            // println!("index is {}, directory_name is {:?}", index, directory_name);
            // let target_index = index || 1;

            // NOTE: probably needs an improvement
            let target_directory = target_directory_list.clone().nth(index).unwrap();
            println!("target_directory is {:?}", &target_directory);
            // .expect("nth element couldnt found");

            // println!("target_directory is {:?}", target_directory);

            match acc.get_mut(&target_directory.to_string()).unwrap() {
                KeyValue::RefCell(something) => something,
                _ => panic!("PANIC"),
            }
        },
    );

    return result;
    // return hashmap;
}

fn find_directory_map_and_insert_file(
    file_system_map: &mut RefCell<HashMap<String, KeyValue>>,
    path: Display,
) {
    let path_string = path.to_string();
    // let mut target_hash_map = get_from_directory_map_from_map(&mut file_system_map,

    // target_hash_map.
    println!("path is {}", path_string);
}
