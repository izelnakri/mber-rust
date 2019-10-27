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
    console::log(format!("{} memserver.js...", Paint::yellow("BUILDING:")));

    let build_start = Instant::now();
    let project_root = &config.project_root.display();
    let output_path = PathBuf::from_str(format!("{}/tmp/assets/memserver.js", &project_root).as_str())?;
    let memserver_path = PathBuf::from_str(format!("{}/memserver", &project_root).as_str())?;
    let should_minify = vec!["production", "demo"].contains(&config.env["environment"].as_str().unwrap());
    let user_memserver_code = recursive_file_lookup::lookup_for_extensions_and_predicate(
        &memserver_path,
        vec![".js", ".ts", ".hbs"],
        |entry| { return !entry.file_name().to_str().unwrap().ends_with("-test.js"); }
    ).into_iter()
    .map(|file| transpilers::convert_es_module::from_file(&file, should_minify))
    .collect::<Vec<&str>>()
    .join("\n");
    let memserver_vendor_code = include_bytes!("../../_vendor/memserver.js");
    let memserver_instance_initializer_code = transpilers::convert_es_module::from_string(
        include_bytes!("../../_vendor/mber-memserver/instance-initializer/memserver.js"),
        project_root, // NOTE: this should change
        should_minify
    );

    fs::write(&output_path, format!(
        "define = window.define; {}",
        vec![memserver_vendor_code, memserver_instance_initializer_code, user_memserver_code].join("\n")
    ))?; // TODO: maybe minify here on demand

    // TODO: in future create a thread global build error to say/stop tts on error

    let output_metadata = fs::metadata(output_path)?;
    let message = format!(
        "{} memserver.js in {} [{}] Environment: {}",
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

    const MEMSERVER_JS_BUILD_TIME_THRESHOLD: u32 = 2000;
    const MEMSERVER_JS_TARGET_BYTE_SIZE: u64 = 1100;
    const MEMSERVER_JS_COMPRESSED_TARGET_BYTE_SIZE: u64 = 1100;

    fn setup_test() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let memserver_js_output_path = format!("{}/tmp/assets/memserver.js", &project_directory);

        Paint::disable();
        fs::remove_file(&memserver_js_output_path).unwrap_or_else(|_| {});
        env::set_current_dir(&project_directory)?;

        return Ok((current_directory, memserver_js_output_path, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        Paint::enable();
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_works_for_development() -> Result<(), Box<dyn Error>> {
        let (current_directory, memserver_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&memserver_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "moduleprefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config, false)?; // note: config and lint
        let build_time_in_ms = Regex::new(r"memserver\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("memserver.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < MEMSERVER_JS_BUILD_TIME_THRESHOLD);

        // let memserver_js_code = fs::read_to_string(memserver_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(memserver_js_output_path)?.len() >= MEMSERVER_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: memserver\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_production() -> Result<(), Box<dyn Error>> {
        let (current_directory, memserver_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&memserver_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "production", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"memserver\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("memserver.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < MEMSERVER_JS_BUILD_TIME_THRESHOLD);

        // let memserver_js_code = fs::read_to_string(memserver_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(memserver_js_output_path)?.len() >= MEMSERVER_JS_COMPRESSED_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: memserver\.js in \d+ms \[\d+.\d+ kB\] Environment: production")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_custom_environment() -> Result<(), Box<dyn Error>> {
        let (current_directory, memserver_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&memserver_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "custom", "modulePrefix": "my-app" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"memserver\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("memserver.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < MEMSERVER_JS_BUILD_TIME_THRESHOLD);

        // let memserver_js_code = fs::read_to_string(memserver_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(memserver_js_output_path)?.len() >= MEMSERVER_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: memserver\.js in \d+ms \[\d+.\d+ kB\] Environment: custom")?.find(&message).is_some());

        return finalize_test(current_directory);
    }
}
