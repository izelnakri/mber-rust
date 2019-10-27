// cargo:rerun-if-changed=build.rs
use core::str::Split;
use serde::{Deserialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::result::Result;
use std::error::Error;
use std::path::Display;
use walkdir::WalkDir;

#[path = "src/utils/walk_injection.rs"] mod walk_injection;

use walk_injection::KeyValue;

#[derive(Deserialize)]
struct CargoTOML {
    package: CargoPackage,
}

#[derive(Deserialize)]
struct CargoPackage {
    version: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    update_help_command_with_version_from_cargo()?;
    inject_documentation_addon_to_source_code_before_compile()?; // NOTE: hashmap has to be flat! so pathString -> content

    return inject_new_ember_app_to_source_code_before_compile();
}

fn update_help_command_with_version_from_cargo() -> Result<(), Box<dyn Error>> {
    let package_details: CargoTOML =
        toml::from_str(fs::read_to_string("Cargo.toml")?.as_str())?;
    let mber_version = package_details.package.version;
    let help_code = fs::read_to_string("src/commands/help.rs")?;
    let help_code_version_line = help_code
        .lines()
        .find(|line| line.trim_start().starts_with("let version ="))
        .expect("cant find version line!")
        .trim();

    return Ok(fs::write(
        "src/commands/help.rs",
        help_code.replace(
            help_code_version_line,
            format!("let version = \"{}\";", mber_version).as_str(),
        ),
    )?);
}

fn inject_new_ember_app_to_source_code_before_compile() -> Result<(), Box<dyn Error>> {
    let file_system_map = build_hashmap_from_file_directory_with_filter("ember-app-boilerplate", |entry| {
        return match entry.path().display().to_string().as_str() {
            "ember-app-boilerplate/node_modules" | "ember-app-boilerplate/package-lock.json" |
            "ember-app-boilerplate/tmp" => false,
            _ => true
        };
    });
    let json_string = serde_json::to_string(&file_system_map)?;

    return Ok(fs::write(
        "src/injections/new_ember_application.rs",
        format!(
            "pub fn as_str() -> &'static str {{\nreturn r##\"\n{}\"##;\n}}",
            json_string
        ),
    )?);
}

fn inject_documentation_addon_to_source_code_before_compile() -> Result<(), Box<dyn Error>> {
    let file_system_map = build_hashmap_from_file_directory_with_filter("_vendor/mber-documentation", |_| { return true });
    let flat_documentation_fs_hashmap = walk_injection::flatten_fs_hashmap_in_binary(file_system_map.into_inner(), vec![]);
    let json_string = serde_json::to_string(&flat_documentation_fs_hashmap)?;

    return Ok(fs::write(
        "src/injections/documentation.rs",
        format!(
            "pub fn as_str() -> &'static str {{\nreturn r##\"\n{}\"##;\n}}",
            json_string
        ),
    )?);
}

fn build_hashmap_from_file_directory_with_filter<F>(directory_string: &str, filter_function: F) -> RefCell<HashMap<String, KeyValue>>
    where F: FnMut(&walkdir::DirEntry) -> bool {
    let walker = WalkDir::new(directory_string).into_iter().filter_entry(filter_function);
    let mut file_system_map: RefCell<HashMap<String, KeyValue>> = RefCell::new(HashMap::new());

    for entry in walker {
        let entry = entry.unwrap();

        match entry.file_type().is_dir() {
            true => check_and_set_directory_map_to_map(&mut file_system_map, entry.path().display()),
            false => find_directory_map_and_insert_file(&mut file_system_map, entry.path().display())
        }
    }

    return file_system_map;
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
                )
            }
        },
    );
}

fn find_directory_map_and_insert_file(
    file_system_map: &mut RefCell<HashMap<String, KeyValue>>,
    path: Display,
) {
    let path_string = path.to_string();
    let file_name = &path_string.split("/").last().unwrap();
    let path_list = &path_string.split("/");
    let directory_path_index = &path_list.clone().count().wrapping_sub(1);
    let contents = fs::read(&path_string).expect(format!("couldnt read {}", path_string).as_str());
    let mut full_map = file_system_map.borrow_mut();
    let target_hash_map =
        get_from_directory_map_from_map(&mut full_map, &path_list, *directory_path_index);

    target_hash_map.insert(file_name.to_string(), KeyValue::Vec(contents));
}
