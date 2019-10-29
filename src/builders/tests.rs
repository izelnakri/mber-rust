use std::time::Instant;
use std::path::PathBuf;
use std::str::FromStr;
use std::result::Result;
use std::error::Error;
use std::fs;
use yansi::Paint;
use super::super::utils::{console, recursive_file_lookup, file};
use super::super::transpilers;
use super::super::types::Config;

// NOTE: eslint in rust(This one is challenging)
pub fn build(config: Config, _lint: bool) -> Result<(String, fs::Metadata), Box<dyn Error>> {
    console::log(format!("{} tests.js...", Paint::yellow("BUILDING:")));

    let build_start = Instant::now();
    let project_root = &config.project_root.display();
    let output_path = PathBuf::from_str(format!("{}/tmp/assets/tests.js", &project_root).as_str())?;
    let should_minify = vec!["production", "demo"].contains(&config.env["environment"].as_str().unwrap());
    let tests_folder_code = recursive_file_lookup::lookup_for_extensions(
        &PathBuf::from_str(format!("{}/tests", &project_root).as_str())?,
        vec![".js", ".ts"]
    ).into_iter()
    .map(|file| transpilers::convert_es_module::from_file(&file, should_minify))
    .collect::<Vec<&str>>()
    .join("\n");
    let app_folder_test_code = recursive_file_lookup::lookup_for_extensions_and_predicate(
        &PathBuf::from_str(format!("{}/src", &project_root).as_str())?,
        vec!["js", "ts"],
        |entry| {
            let file_name = entry.file_name().to_str().unwrap();

            return file_name.ends_with("-test.js") || file_name.ends_with("-test.ts");
        }
    ).into_iter()
    .map(|file| transpilers::convert_es_module::from_file(&file, should_minify))
    .collect::<Vec<&str>>()
    .join("\n");
    let code = format!(
        "define = window.define; {}
        {}
        window.require('{}/tests/test-helper');
        EmberENV.TESTS_FILE_LOADED = true;
        {}
        ", config.build_cache.test_prepends, vec![tests_folder_code, app_folder_test_code].join("\n"),
        config.application_name, config.build_cache.test_appends);

    fs::write(&output_path, code)?;

    // TODO: in future create a thread global build error to say/stop tts on error

    let output_metadata = fs::metadata(output_path)?;
    let message = format!(
        "{} tests.js in {} [{}] Environment: {}",
        Paint::green("BUILT:"),
        Paint::yellow(file::format_time_passed(build_start.elapsed().as_millis())),
        file::format_size(output_metadata.len()),
        &config.env["environment"].as_str().unwrap()
    );

    console::log(&message);

    // NOTE: then linting

    return Ok((message, output_metadata));
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use std::path::PathBuf;
    use regex::Regex;
    use serde_json::json;
    use std::collections::HashMap;
    use super::super::super::types::BuildCache;

    const TESTS_JS_BUILD_TIME_THRESHOLD: u32 = 2000;
    const TESTS_JS_TARGET_BYTE_SIZE: u64 = 1100;
    const TESTS_JS_COMPRESSED_TARGET_BYTE_SIZE: u64 = 1100;
    const CODE_TO_PREPEND: &str = "(function() { console.log('this is prepending code') })()";
    const CODE_TO_APPEND: &str = "(function() { console.log('this is appending code') })()";

    fn setup_test() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let tests_js_output_path = format!("{}/tmp/assets/tests.js", &project_directory);

        Paint::disable();
        fs::remove_file(&tests_js_output_path).unwrap_or_else(|_| {});
        env::set_current_dir(&project_directory)?;

        return Ok((current_directory, tests_js_output_path, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        Paint::enable();
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_works_for_development() -> Result<(), Box<dyn Error>> {
        let (current_directory, tests_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&tests_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "moduleprefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"tests\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("tests.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < TESTS_JS_BUILD_TIME_THRESHOLD);

        // let test_js_code = fs::read_to_string(tests_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(tests_js_output_path)?.len() >= TESTS_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: tests\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_test_environment() -> Result<(), Box<dyn Error>> {
        let (current_directory, tests_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&tests_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "production", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"tests\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("tests.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < TESTS_JS_BUILD_TIME_THRESHOLD);

        // let test_js_code = fs::read_to_string(tests_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(tests_js_output_path)?.len() >= TESTS_JS_COMPRESSED_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: tests\.js in \d+ms \[\d+.\d+ kB\] Environment: production")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_custom_environment() -> Result<(), Box<dyn Error>> {
        let (current_directory, tests_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&tests_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "custom", "modulePrefix": "my-app" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"tests\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("tests.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < TESTS_JS_BUILD_TIME_THRESHOLD);

        // let test_js_code = fs::read_to_string(tests_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(tests_js_output_path)?.len() >= TESTS_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: tests\.js in \d+ms \[\d+.\d+ kB\] Environment: custom")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_test_prepends() -> Result<(), Box<dyn Error>> {
        let (current_directory, tests_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&tests_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new().insert("test_prepends", CODE_TO_PREPEND)
        );
        let (message, _stats) = build(config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"tests\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("tests.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let tests_js_code = fs::read_to_string(&tests_js_output_path)?;

        assert!(build_time_in_ms < TESTS_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(tests_js_output_path)?.len() >= TESTS_JS_TARGET_BYTE_SIZE - 1000);
        assert!(tests_js_code.contains(CODE_TO_PREPEND));
        assert!(!tests_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: tests\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_test_appends() -> Result<(), Box<dyn Error>> {
        let (current_directory, tests_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&tests_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new().insert("test_appends", CODE_TO_APPEND)
        );
        let (message, _stats) = build(config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"tests\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("tests.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let tests_js_code = fs::read_to_string(&tests_js_output_path)?;

        assert!(build_time_in_ms < TESTS_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(tests_js_output_path)?.len() >= TESTS_JS_TARGET_BYTE_SIZE - 1000);
        assert!(!tests_js_code.contains(CODE_TO_PREPEND));
        assert!(tests_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: tests\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_test_appends_and_prepends() -> Result<(), Box<dyn Error>> {
        let (current_directory, tests_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&tests_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
                .insert("test_prepends", CODE_TO_PREPEND)
                .insert("test_appends", CODE_TO_APPEND)
        );
        let (message, _stats) = build(config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"tests\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("tests.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let tests_js_code = fs::read_to_string(&tests_js_output_path)?;

        assert!(build_time_in_ms < TESTS_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(tests_js_output_path)?.len() >= TESTS_JS_TARGET_BYTE_SIZE - 1000);
        assert!(tests_js_code.contains(CODE_TO_PREPEND));
        assert!(tests_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: tests\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }
}
