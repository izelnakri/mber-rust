use std::time::Instant;
use std::path::PathBuf;
use std::str::FromStr;
use std::result::Result;
use std::error::Error;
use std::fs;
use yansi::Paint;
use serde_json::{value::Value};
use super::super::utils::{console, file};
use super::super::transpilers::{import_addon_folder_to_amd}; // also convert_es_module
use super::super::types::Config;

// NOTE: has hard dependency on ember-data(when needed) and ember-cli-fastboot
// TODO: content/module check tests
pub fn build(config: Config) -> Result<(String, fs::Metadata), Box<dyn Error>> {
    console::log(format!("{} vendor.js...", Paint::yellow("BUILDING:")));

    let build_start = Instant::now();
    let project_root = &config.project_root.display();
    let output_path = PathBuf::from_str(format!("{}/tmp/assets/vendor.js", &project_root).as_str())?;
    let _should_minify = vec!["production", "demo"].contains(&config.env["environment"].as_str().unwrap());
    let should_exclude_ember_data = &config.env["excludeEmberData"].as_bool().unwrap_or(false);

    let mut content = vec![
        get_right_ember_base_string(&config.env, &should_exclude_ember_data),
        if should_exclude_ember_data == &true {
            String::from("")
        } else {
            import_addon_folder_to_amd::to_string("ember-data/app", &config)
        },
        String::from_utf8(include_bytes!("../../_vendor/mber-documentation/index.js").to_vec())?
    ].join("\n");

    if config.cli_arguments.fastboot {
        let fastboot_initializer_code = match &config.env["memserver"]["enabled"].as_bool() { // TODO: this needs to be converted with convert_es_module::to_amd as initializer
            Some(_) => String::from_utf8(include_bytes!("../../_vendor/memserver/fastboot/initializers/ajax.js").to_vec())?,
            None => String::from_utf8(include_bytes!("../../_vendor/fastboot/initializers/ajax.js").to_vec())?
        };
        content.push_str(vec![
            String::from_utf8(include_bytes!("../../_vendor/fastboot/fastboot-addon-modules.js").to_vec())?,
            String::from_utf8(include_bytes!("../../_vendor/fetch/fetch-fastboot-shim.js").to_vec())?,
            fastboot_initializer_code,
            import_addon_folder_to_amd::to_string("ember-cli-fastboot/app", &config)
        ].join("\n").as_str());
    }

    fs::write(&output_path, format!("{}
        window.EmberENV = JSON.parse({});
        window.runningTests = !!(window.location && (window.location.pathname === '/tests') && (EmberENV.environment !== 'production'));
        {}
        {}
        {}
    ", &config.build_cache.vendor_prepends, &config.env.to_string(), content,
    add_socket_watch_code(&config.cli_arguments.port), &config.build_cache.vendor_appends))?; // TODO: maybe minify here on demand

    // TODO: in future create a thread global build error to say/stop tts on error

    let output_metadata = fs::metadata(output_path)?;
    let message = format!(
        "{} vendor.js in {} [{}] Environment: {}",
        Paint::green("BUILT:"),
        Paint::yellow(file::format_time_passed(build_start.elapsed().as_millis())),
        file::format_size(output_metadata.len()),
        &config.env["environment"].as_str().unwrap()
    );

    console::log(&message);

    return Ok((message, output_metadata));
}

fn get_right_ember_base_string(env: &Value, should_exclude_ember_data: &bool) -> String {
    match (should_exclude_ember_data, vec!["production", "demo"].contains(&env["environment"].as_str().unwrap())) {
        (true, true) => String::from_utf8(include_bytes!("../../_vendor/no-ember-data-ember-prod.js").to_vec()).unwrap(),
        (true, false) => String::from_utf8(include_bytes!("../../_vendor/no-ember-data-ember-debug.js").to_vec()).unwrap(),
        (false, true) => String::from_utf8(include_bytes!("../../_vendor/full-ember-prod.js").to_vec()).unwrap(),
        (false, false) => String::from_utf8(include_bytes!("../../_vendor/full-ember-debug.js").to_vec()).unwrap()
    }
}

fn add_socket_watch_code(socket_port: &u16) -> String {
  return format!("
    if (typeof FastBoot === 'undefined') {{
      window.socket = new WebSocket('ws://localhost:{}');

      window.socket.addEventListener('message', function(event) {{
        document.querySelectorAll('.ember-view').forEach((e) => e.remove());
        window.location.reload(true);
      }});
    }}
  ", socket_port.to_string());
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

    const VENDOR_JS_BUILD_TIME_THRESHOLD: u32 = 2000;
    const VENDOR_JS_TARGET_BYTE_SIZE: u64 = 1100;
    const VENDOR_JS_COMPRESSED_TARGET_BYTE_SIZE: u64 = 1100;
    const CODE_TO_PREPEND: &str = "(function() { console.log('this is prepending code') })()";
    const CODE_TO_APPEND: &str = "(function() { console.log('this is appending code') })()";

    fn setup_test() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let vendor_js_output_path = format!("{}/tmp/assets/vendor.js", &project_directory);

        Paint::disable();
        fs::remove_file(&vendor_js_output_path).unwrap_or_else(|_| {});
        env::set_current_dir(&project_directory)?;

        return Ok((current_directory, vendor_js_output_path, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        Paint::enable();
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_works_for_development() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "moduleprefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);

        // let vendor_js_code = fs::read_to_string(vendor_js_output_path).unwrap();
        // todo: content checks

        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ MB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_production() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "production", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);

        // let vendor_js_code = fs::read_to_string(vendor_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_COMPRESSED_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ kB\] Environment: production")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_development_without_ember_data() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "excludeEmberData": true, "environment": "development", "moduleprefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);

        // let vendor_js_code = fs::read_to_string(vendor_js_output_path).unwrap();
        // todo: content checks

        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ MB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_production_without_ember_data() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "excludeEmberData": true, "environment": "production", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);

        // let vendor_js_code = fs::read_to_string(vendor_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_COMPRESSED_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ kB\] Environment: production")?.find(&message).is_some());

        return finalize_test(current_directory);
    }


    #[test]
    fn build_works_for_development_without_fastboot() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);


        let mut config = Config::build(
            json!({ "environment": "development", "moduleprefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );

        config.cli_arguments.fastboot = false;

        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);

        // let vendor_js_code = fs::read_to_string(vendor_js_output_path).unwrap();
        // todo: content checks

        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ MB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_production_without_fastboot() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let mut config = Config::build(
            json!({ "environment": "production", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );

        config.cli_arguments.fastboot = false;

        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);

        // let vendor_js_code = fs::read_to_string(vendor_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_COMPRESSED_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ kB\] Environment: production")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_custom() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "custom", "moduleprefix": "my-app" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);

        // let vendor_js_code = fs::read_to_string(vendor_js_output_path).unwrap();
        // todo: content checks

        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ MB\] Environment: custom")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_custom_without_fastboot() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let mut config = Config::build(
            json!({ "environment": "custom", "moduleprefix": "my-app" }),
            HashMap::new(),
            BuildCache::new()
        );

        config.cli_arguments.fastboot = false;

        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);

        // let vendor_js_code = fs::read_to_string(vendor_js_output_path).unwrap();
        // todo: content checks

        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ MB\] Environment: custom")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_application_prepends() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new().insert("vendor_prepends", CODE_TO_PREPEND)
        );
        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let vendor_js_code = fs::read_to_string(&vendor_js_output_path)?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_TARGET_BYTE_SIZE - 1000);
        assert!(vendor_js_code.contains(CODE_TO_PREPEND));
        assert!(!vendor_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ MB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_application_appends() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new().insert("vendor_appends", CODE_TO_APPEND)
        );
        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let vendor_js_code = fs::read_to_string(&vendor_js_output_path)?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_TARGET_BYTE_SIZE - 1000);
        assert!(!vendor_js_code.contains(CODE_TO_PREPEND));
        assert!(vendor_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ MB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_application_appends_and_prepends() -> Result<(), Box<dyn Error>> {
        let (current_directory, vendor_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&vendor_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
                .insert("vendor_prepends", CODE_TO_PREPEND)
                .insert("vendor_appends", CODE_TO_APPEND)
        );
        let (message, _stats) = build(config)?;
        let build_time_in_ms = Regex::new(r"vendor\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("vendor.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let vendor_js_code = fs::read_to_string(&vendor_js_output_path)?;

        assert!(build_time_in_ms < VENDOR_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(vendor_js_output_path)?.len() >= VENDOR_JS_TARGET_BYTE_SIZE - 1000);
        assert!(vendor_js_code.contains(CODE_TO_PREPEND));
        assert!(vendor_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: vendor\.js in \d+ms \[\d+.\d+ MB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }
}
