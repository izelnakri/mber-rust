use std::time::Instant;
use std::path::PathBuf;
use std::str::FromStr;
use std::error::Error;
use std::fs;
use yansi::Paint;
use super::super::utils::{console, recursive_file_lookup, file, walk_injection};
use super::super::transpilers;
use super::super::injections::documentation;
use super::super::types::Config;

// NOTE: eslint in rust(This one is challenging)
pub fn build(config: &Config, _lint: bool) -> Result<(String, fs::Metadata), Box<dyn Error>> {
    console::log(format!("{} documentation.js...", Paint::yellow("BUILDING:")));

    let build_start = Instant::now();
    let project_root = &config.project_root.display();
    let output_path = PathBuf::from_str(format!("{}/tmp/assets/documentation.js", &project_root).as_str())?;
    let documentation_path = PathBuf::from_str(format!("{}/documentation", &project_root).as_str())?;
    let should_minify = vec!["production", "demo"]
        .contains(&config.env["environment"].as_str().expect("ENV.environment not found on config/environment.js"));
    let documentation_addon_code = import_documentation_code(&config.project_root, &config.application_name, should_minify);
    let contents = recursive_file_lookup::lookup_for_extensions(
        &documentation_path,
        vec![".js", ".ts", ".hbs"]
    ).into_iter()
    .map(|file| transpilers::convert_es_module::from_file(&file, should_minify))
    .collect::<Vec<&str>>()
    .join("\n");

    fs::write(&output_path, format!(
        "define = window.define; {}",
        vec![documentation_addon_code, contents].join("\n")
    ))?; // TODO: maybe minify here on demand

    // TODO: in future create a thread global build error to say/stop tts on error

    let output_metadata = fs::metadata(output_path)?;
    let message = format!(
        "{} documentation.js in {} [{}] Environment: {}",
        Paint::green("BUILT:"),
        Paint::yellow(file::format_time_passed(build_start.elapsed().as_millis())),
        file::format_size(output_metadata.len()),
        &config.env["environment"].as_str().unwrap()
    );

    console::log(&message);

    // NOTE: then linting

    return Ok((message, output_metadata));
}

fn import_documentation_code(_project_root: &PathBuf, application_name: &String, should_minify: bool) -> String {
    let documentation_hashmap = serde_json::from_str(documentation::as_str()).unwrap(); // TODO: always keep it flat
    let flat_documentation_hashmap = walk_injection::flatten_fs_hashmap(documentation_hashmap, vec![]);

    return format!(
        "{} {} {}",
        flat_documentation_hashmap.get("_vendor/mber-documentation/vendor/copee.umd.js").unwrap().clone(),
        flat_documentation_hashmap.get("_vendor/mber-documentation/vendor/highlight.pack.js").unwrap().clone(),
        walk_injection::lookup_for_extensions_with_predicate(flat_documentation_hashmap, vec![".js", ".ts", ".hbs"], |filename| {
            return filename.starts_with("_vendor/mber-documentation/src");
        }).iter()
        .map(|(_path, content)| {
            return transpilers::convert_es_module::from_string(content, application_name, should_minify);
        }) // NOTE: build the right module path and hbs transpiler if it needs it
        .collect::<Vec<&str>>()
        .join("\n")
    );
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

    const BUILD_TIME_THRESHOLD: u32 = 2000;

    fn setup_test() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let documentation_js_output_path = format!("{}/tmp/assets/documentation.js", &project_directory);

        Paint::disable();
        fs::remove_file(&documentation_js_output_path).unwrap_or_else(|_| {});
        env::set_current_dir(&project_directory)?;
        fs::create_dir_all("tmp/assets").unwrap_or_else(|_| {});

        return Ok((current_directory, documentation_js_output_path, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        Paint::enable();
        fs::remove_dir_all("tmp").unwrap_or_else(|_| {});
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_works_for_development_and_production() -> Result<(), Box<dyn Error>> {
        let (current_directory, documentation_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&documentation_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (development_build_message, _stats) = build(&config, false)?;
        let development_build_time_in_ms = Regex::new(r"documentation\.js in \d+ms")?
            .find(development_build_message.as_str()).unwrap().as_str()
            .replace("documentation.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let development_build_output_size = fs::metadata(&documentation_js_output_path)?.len();

        assert!(development_build_time_in_ms < BUILD_TIME_THRESHOLD);
        assert!(development_build_output_size >= 100);
        assert!(
            Regex::new(r"BUILT: documentation\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?
                .find(&development_build_message).is_some()
        );

        finalize_test(current_directory)?;
        let (current_directory, documentation_js_output_path, _) = setup_test()?;

        let production_config = Config::build(
            json!({ "environment": "production", "modulePrefix": "customapp" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (production_build_message, _stats) = build(&production_config, false)?;
        let production_build_time_in_ms = Regex::new(r"documentation\.js in \d+ms")?
            .find(production_build_message.as_str()).unwrap().as_str()
            .replace("documentation.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let production_build_output_size = fs::metadata(documentation_js_output_path)?.len();

        assert!(production_build_time_in_ms < BUILD_TIME_THRESHOLD);
        assert!(production_build_output_size >= 100);
        // assert!(development_build_output_size > production_build_output_size);
        assert!(
            Regex::new(r"BUILT: documentation\.js in \d+ms \[\d+.\d+ kB\] Environment: production")?
                .find(&production_build_message).is_some()
        );

        return finalize_test(current_directory);
    }
}
