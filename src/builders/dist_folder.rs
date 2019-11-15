use std::time::Instant;
use std::fs;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use yansi::Paint;
use md5;
use serde_json;
use serde_json::{Map, json, Value};
use select::document::Document;
use super::{index_html, fastboot_package_json};
use super::super::utils::{console, file, html_file, project};
use super::super::types::Config;

pub fn build(config: &Config) -> Result<(String, Vec<Value>), Box<dyn Error>> {
    console::log(format!("{} {}...", Paint::yellow("BUNDLING:"), config.application_name));

    let bundle_start = Instant::now();
    let environment = config.env["environment"].as_str().unwrap_or("development");
    let project_root = &config.project_root.display();
    let output_directory = format!("{}/dist", &project_root);
    let should_build_tests = (environment != "production") && config.cli_arguments.testing;
    let should_build_documentation = config.env["documentation"]["enabled"].as_bool().unwrap_or(false);

    reset_output_folder(&output_directory.as_str())?;

    let mut build_files: Vec<String> = Vec::new();
    let index_html_path_tuple = (format!("{}/index.html", &project_root), format!("{}/dist/index.html", &project_root));

    build_files.extend(get_build_files_from_html(&index_html_path_tuple.0.as_str(), &config, false)?);

    let mut build_html_paths = vec![index_html_path_tuple];

    if should_build_tests {
        let tests_html_path_tuple = (
            format!("{}/tests/index.html", &project_root), format!("{}/dist/tests.html", &project_root)
        );

        build_files.extend(get_build_files_from_html(&tests_html_path_tuple.0.as_str(), &config, false)?);
        build_html_paths.push(tests_html_path_tuple);
    }

    if should_build_documentation {
        let documentation_path_in_config = config.env["documentation"]["path"].as_str().unwrap_or("styleguide");
        let documentation_html_path_tuple = (
            format!("{}/tmp{}.html", &config.project_root.display(), documentation_path_in_config),
            format!("{}/dist{}.html", &project_root, documentation_path_in_config)
        );

        build_files.extend(get_build_files_from_html(&documentation_html_path_tuple.0.as_str(), &config, true)?);
        build_html_paths.push(documentation_html_path_tuple);
    }

    build_files.sort();
    build_files.dedup();

    let target_asset_map = build_files.iter().fold(HashMap::new(), |mut result, file_name| {
        let content = fs::read_to_string(format!("{}/tmp{}", &project_root, &file_name)).unwrap();

        result.insert(file_name, content);

        return result;
    });
    let hashed_file_name_map = build_hashed_filename_map(&target_asset_map);

    project::recursively_copy_folder(format!("{}/public", &project_root), &output_directory)?;
    safe_write_html_and_assets(&output_directory, build_html_paths, &hashed_file_name_map, &target_asset_map)?;

    let target_map_json = build_file_map_with_asset_map(hashed_file_name_map);

    write_asset_map(&config.project_root, &target_map_json)?;

    if config.cli_arguments.fastboot {
        fastboot_package_json::build(target_map_json, config, Some("dist"))?;
    }

    // TODO: in future create a thread global build error to say/stop tts on error

    let build_message = format!(
        "{} {} in {}",
        Paint::green("BUNDLED:"),
        &config.application_name,
        Paint::yellow(file::format_time_passed(bundle_start.elapsed().as_millis())),
    );

    console::log(&build_message);
    console::log(Paint::green("Built project successfully. Stored in \"./dist\":"));

    let dist_assets = fs::read_dir(format!("{}/assets", output_directory))?;
    let output_metadata = dist_assets.fold(Vec::new(), |mut result, entry| {
        let target_entry = entry.unwrap();
        let file_name = String::from(target_entry.file_name().to_str().unwrap());

        if file_name.ends_with(".js") || file_name.ends_with(".css") {
            let file_size = target_entry.metadata().unwrap().len();
            let gzip_size = file::gzip_metadata(&target_entry.path()).unwrap();
            let file_metadata = json!({
                "file_name": file_name,
                "size": file_size,
                "gzip_size": gzip_size
            });

            println!(
                "{} {} {}",
                Paint::blue(format!(" - {}:", file_metadata["file_name"].as_str().unwrap())),
                Paint::yellow(file::format_size(file_size)),
                Paint::green(format!("[{} gzipped]", file::format_size(gzip_size as u64)))
            );

            result.push(file_metadata);
        }

        return result;
    });

    return Ok((build_message, output_metadata));
}

fn reset_output_folder(output_directory: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all(output_directory).unwrap_or_else(|_| {});

    return Ok(fs::create_dir_all(format!("{}/assets", output_directory).as_str())?);
}

fn get_build_files_from_html(html_path: &str, config: &Config, is_documentation: bool)
    -> Result<Vec<String>, Box<dyn Error>> {
    let html = match is_documentation {
        true => index_html::build_documentation_html(&html_path, &config)?,
        false => index_html::build(&html_path, &config)?
    };
    let html_document = Document::from(html.as_str());
    let (html_js_files, html_css_files) = html_file::find_internal_assets_from_html(&html_document);

    return Ok(html_js_files.into_iter().chain(html_css_files.into_iter()).collect());
}

fn build_hashed_filename_map<'a>(asset_map: &'a HashMap<&String, String>) -> HashMap<&'a String, String> {
    return asset_map.iter().fold(HashMap::new(), |mut result, (file_name, content)| {
        let hash = format!("{:?}", md5::compute(content));
        let file = PathBuf::from(file_name);
        let file_reference = file.iter().fold(String::new(), |mut result, path_component| {
            if path_component == file.file_name().unwrap() {
                result.push_str(format!("/{}", file.file_stem().unwrap().to_str().unwrap()).as_str());
            } else {
                result.push_str(path_component.to_str().unwrap());
            }

            return result;
        });

        result.insert(file_name, format!("{}-{}.{}", file_reference, hash, file.extension().unwrap().to_str().unwrap()));

        return result;
    });
}

fn safe_write_html_and_assets(
    output_directory: &String,
    html_path_tuples: Vec<(String, String)>,
    hashed_file_names: &HashMap<&String, String>,
    target_asset_map: &HashMap<&String, String>
) -> Result<(), Box<dyn Error>> {
    html_path_tuples.iter().for_each(|(html_path, target_dist_html_path)| {
        let html_content = fs::read_to_string(&html_path).unwrap();
        let target_content = hashed_file_names.iter().fold(html_content, |result, (file_name, hashed_file_name)| {
            return result.replace(file_name.as_str(), hashed_file_name.as_str());
        });

        fs::write(target_dist_html_path, target_content).unwrap();
    });
    target_asset_map.iter().for_each(|(file_name, content)| {
        let hashed_file_name = hashed_file_names.get(file_name).unwrap();

        fs::write(format!("{}/{}", &output_directory, hashed_file_name), content).unwrap();
    });

    return Ok(());
}

fn build_file_map_with_asset_map<'a>(hashed_file_name_map: HashMap<&'a String, String>) -> Value {
    let mut map = Map::new();

    hashed_file_name_map.iter().for_each(|(key, value)| {
        map.insert(key[1..].to_string(), Value::String(value[1..].to_string()));
    });
    map.insert("assets/assetMap.json".to_string(), Value::String("assets/assetMap.json".to_string()));

    return Value::Object(map);
}

fn write_asset_map(project_root: &PathBuf, hashed_file_names: &Value)
    -> Result<(), Box<dyn Error>> {
    fs::write(format!("{}/dist/assets/assetMap.json", project_root.display()), serde_json::to_string_pretty(&json!({
        "assets": hashed_file_names,
        "prepend": ""
    })).unwrap())?;

    return Ok(());
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use std::path::PathBuf;
    use serde_json::json;
    use std::collections::HashMap;
    use super::super::{build_all_assets};
    use super::super::super::types::BuildCache;

    const TIME_TO_BUILD_DIST_THRESHOLD: u128 = 4000;

    fn get_file_key(json_value: &Value, key: &str) -> String {
        match &json_value.get(key) {
            Some(value) => format!("/{}", value.as_str().unwrap_or("")),
            None => String::new()
        }
    }

    fn setup_test() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());

        env::set_current_dir(&project_directory)?;

        let output_directory = format!("{}/dist", &project_directory);

        fs::remove_dir_all(&output_directory).unwrap_or_else(|_| {});
        fs::remove_dir_all("tmp").unwrap_or_else(|_| {});

        return Ok((current_directory, output_directory, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        fs::remove_dir_all("tmp")?;
        fs::remove_dir_all("dist")?;
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_dist_folder_works() -> Result<(), Box<dyn Error>> {
        let (actual_current_directory, output_directory, project_directory) = setup_test()?;

        assert_eq!(fs::metadata(&output_directory).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        ); // NOTE: testing: true must be there

        build_all_assets(&config)?;

        let build_start = Instant::now();
        let (message, build_metadata_output) = build(&config)?;
        let time_passed = build_start.elapsed().as_millis();
        let file_names: Vec<PathBuf> = fs::read_dir("dist/assets")?.fold(Vec::new(), |mut result, entry| {
            let dir_entry = entry.unwrap();

            if dir_entry.metadata().unwrap().is_file() {
                result.push(dir_entry.path());
            }

            return result;
        });

        assert!(time_passed < TIME_TO_BUILD_DIST_THRESHOLD);
        assert!(file_names.len() == 7);

        let target_index_html_assets = file_names.iter().filter(|file_name| {
            let target_file_name = file_name.to_str().unwrap().to_string();

            return !target_file_name.contains("tests") && !target_file_name.contains("test-support");
        });
        let output_html = fs::read_to_string("dist/index.html")?;
        let output_test_html = fs::read_to_string("dist/tests.html")?;
        let dist_file_assets = target_index_html_assets.fold(Vec::new(), |mut result, file_name| {
            let file_path = file_name.to_str().unwrap().to_string();
            let target_reference = file_path.replace("./", "").replace("dist/", "/");

            if target_reference != "/assets/assetMap.json" {
                assert!(&output_html.contains(&target_reference));
                assert!(&output_test_html.contains(&target_reference));

                result.push(file_path);
            }

            return result;
        });
        let file_contents = [
            fs::read_to_string("tmp/assets/vendor.js")?,
            fs::read_to_string("tmp/assets/application.css")?,
            fs::read_to_string("tmp/assets/application.js")?,
            fs::read_to_string("tmp/assets/test-support.css")?,
            fs::read_to_string("tmp/assets/test-support.js")?,
            fs::read_to_string("tmp/assets/tests.js")?
        ];

        dist_file_assets.iter().for_each(|dist_file| {
            assert!(file_contents.contains(&fs::read_to_string(dist_file).unwrap()));
        });
        build_metadata_output.iter().for_each(|(file)| {
            let file_size = file["size"].as_u64().unwrap();

            assert!(file_size > 0);

            if file_size > 1000 {
                assert!(file["gzip_size"].as_u64().unwrap() < file_size);
            }
        });

        assert!(fs::metadata("dist/package.json").is_ok());

        let asset_map: Value = serde_json::from_str(fs::read_to_string("dist/assets/assetMap.json")?.as_str())?;

        assert_eq!(asset_map["prepend"], Value::String("".to_string()));
        assert_eq!(asset_map["assets"].as_object().unwrap().len(), 7);
        assert_eq!(asset_map["assets"]["assets/assetMap.json"], Value::String("assets/assetMap.json".to_string()));

        let mut dist_files: Vec<String> = file_names.iter()
            .map(|dist_file| dist_file.to_str().unwrap().to_string().replace("./", "").replace("dist/", "/"))
            .collect();
        let target_assets = &asset_map["assets"];

        assert!(&dist_files.contains(&get_file_key(target_assets, "assets/application.css")));
        assert!(&dist_files.contains(&get_file_key(target_assets, "assets/application.js")));
        assert!(&dist_files.contains(&get_file_key(target_assets, "assets/vendor.js")));
        assert!(!&dist_files.contains(&get_file_key(target_assets, "assets/memserver.js")));
        assert!(&dist_files.contains(&get_file_key(target_assets, "assets/test-support.js")));
        assert!(&dist_files.contains(&get_file_key(target_assets, "assets/test-support.css")));
        assert!(&dist_files.contains(&get_file_key(target_assets, "assets/tests.js")));

        return finalize_test(actual_current_directory);
    }

    // #[test]
    // fn build_works_for_different_application_with_memserver_mode() {
    // }

    // #[test]
    // fn build_works_for_production() {
    // }

    // #[test]
    // fn build_works_for_different_application_with_memserver_mode_and_fastboot_false() {
    // }

    // #[test]
    // fn build_resets_dist() {
    // }
}
