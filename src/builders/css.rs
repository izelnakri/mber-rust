use std::time::Instant;
use std::path::PathBuf;
use std::str::FromStr;
use std::result::Result;
use std::error::Error;
use std::fs;
use yansi::Paint;
use sass_rs;
use super::super::utils::{console, recursive_file_lookup, file};
use super::super::types::Config;

pub fn build(config: &Config) -> Result<(String, fs::Metadata), Box<dyn Error>> {
    console::log(format!("{} application.css...", Paint::yellow("BUILDING:")));

    let build_start = Instant::now();
    let environment = config.env["environment"].as_str().unwrap_or("development");
    let project_root = &config.project_root.display();
    let output_path = PathBuf::from_str(format!("{}/tmp/assets/application.css", &project_root).as_str())?;
    let application_path = PathBuf::from_str(format!("{}/src", &project_root).as_str())?;
    let mut all_styles = vec![fs::read_to_string(format!("{}/src/ui/styles/application.scss", project_root))?];
    let mut component_styles = recursive_file_lookup::lookup_for_extensions_and_predicate(
        &application_path,
        vec![".scss"],
        |entry| { return !entry.file_name().to_str().unwrap().contains("/src/ui/styles"); }
    ).into_iter()
    .map(|file_name| fs::read_to_string(file_name).unwrap())
    .collect::<Vec<String>>();
    let output_style = match vec!["production", "demo"].contains(&environment) {
        true => sass_rs::OutputStyle::Compressed,
        false => sass_rs::OutputStyle::Expanded
    };

    all_styles.append(&mut component_styles);

    fs::write(&output_path, sass_rs::compile_string(&all_styles.join("\n"), sass_rs::Options {
        output_style: output_style, precision: 5, indented_syntax: false,
        include_paths: vec![format!("{}/src/ui/styles", project_root)]
    })?)?;

    // TODO: in future create a thread global build error to say/stop tts on error

    let output_metadata = fs::metadata(output_path)?;
    let message = format!(
        "{} application.css in {} [{}] Environment: {}",
        Paint::green("BUILT:"),
        Paint::yellow(file::format_time_passed(build_start.elapsed().as_millis())),
        file::format_size(output_metadata.len()),
        environment
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

    const CSS_BUILD_TIME_THRESHOLD: u32 = 2000;
    const CSS_TARGET_BYTE_SIZE: u64 = 1100;
    const CSS_COMPRESSED_TARGET_BYTE_SIZE: u64 = 1100;

    fn setup_test() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let application_css_output_path = format!("{}/tmp/assets/application.css", &project_directory);

        Paint::disable();
        fs::remove_file(&application_css_output_path).unwrap_or_else(|_| {});
        env::set_current_dir(&project_directory)?;
        fs::create_dir_all("tmp/assets").unwrap_or_else(|_| {});

        return Ok((current_directory, application_css_output_path, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        Paint::enable();
        fs::remove_dir_all("tmp").unwrap_or_else(|_| {});
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_works_for_development() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_css_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_css_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(&config)?;
        let build_time_in_ms = Regex::new(r"application\.css in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.css in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < CSS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(application_css_output_path)?.len() >= CSS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: application\.css in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_production() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_css_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_css_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "production", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(&config)?;
        let build_time_in_ms = Regex::new(r"application\.css in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.css in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < CSS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(application_css_output_path)?.len() >= CSS_COMPRESSED_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: application\.css in \d+ms \[\d+.\d+ kB\] Environment: production")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_custom_environment() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_css_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_css_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "custom", "modulePrefix": "my-app" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(&config)?;
        let build_time_in_ms = Regex::new(r"application\.css in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.css in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < CSS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(application_css_output_path)?.len() >= CSS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: application\.css in \d+ms \[\d+.\d+ kB\] Environment: custom")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_css_vendor_folder_written_first_then_user_component_css() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_css_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_css_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );

        build(&config)?;

        let application_css_code = fs::read_to_string(application_css_output_path)?;

        assert!(application_css_code.starts_with("#test"));
        assert!(application_css_code.contains("#ember-welcome-page-id-selector"));

        return finalize_test(current_directory);
    }
}
