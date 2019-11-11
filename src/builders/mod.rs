pub mod application;
pub mod css;
pub mod dist_folder;
pub mod documentation_css;
pub mod documentation_js;
pub mod fastboot_package_json;
pub mod index_html;
pub mod memserver;
pub mod test_files;
pub mod vendor;

use std::fs;
use super::types::Config;
use std::error::Error;
use serde_json::{json, Value};

pub fn build_all_assets(config: &Config) -> Result<(), Box<dyn Error>> {
    let project_root = config.project_root.display();

    if !config.cli_arguments.testing {
        fs::remove_dir_all(format!("{}/tmp", &project_root)).unwrap_or_else(|_| {});
    }

    fs::create_dir_all(format!("{}/tmp/assets", &project_root)).unwrap_or_else(|_| {});

    let mut default_asset_map = json!({
      "assets/application.css": "assets/application.css",
      "assets/vendor.js": "assets/vendor.js",
      "assets/application.js": "assets/application.js",
    });
    let memserver_is_enabled = config.env["memserver"]["enabled"].as_bool().unwrap_or(false);
    let documentation_is_enabled = config.env["documentation"]["enabled"].as_bool().unwrap_or(false);
    let index_html_path = format!("{}/index.html", &project_root);

    index_html::build(index_html_path.as_str(), &config)?;
    css::build(&config)?;
    vendor::build(&config)?;
    application::build(&config, false)?; // NOTE: enable linting in future

    if memserver_is_enabled {
        memserver::build(&config, false)?; // NOTE: enable linting in future
        default_asset_map.as_object_mut().unwrap().insert(
            "assets/memserver.js".to_string(),
            Value::String("assets/memserver.js".to_string())
        );
    }

    fastboot_package_json::build(default_asset_map, &config, Some("tmp"))?;

    if documentation_is_enabled {
        documentation_js::build(&config, false)?; // NOTE: enable linting in future
        documentation_css::build(&config)?;

        let test_index_path = format!("{}/tests/index.html", &project_root);

        index_html::build_documentation_html(test_index_path.as_str(), &config)?;
    }

    if config.cli_arguments.testing {
        index_html::build(&index_html_path, &config)?;

        test_files::build(&config, false)?; // NOTE: enable linting in future
        fs::write(format!("{}/tmp/assets/test-support.css", &project_root), include_str!("../../_vendor/test-support.css"))?;
        fs::write(format!("{}/tmp/assets/test-support.js", &project_root), include_str!("../../_vendor/test-support.js"))?;
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use std::path::PathBuf;
    use serde_json::json;
    use std::collections::HashMap;
    use super::super::types::BuildCache;

    fn setup_test() -> Result<(PathBuf, String, String, String, String, String, String, String, String, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());

        env::set_current_dir(&project_directory)?;

        let output_directory = format!("{}/tmp", &project_directory);
        let application_js_output_path = format!("{}/assets/application.js", &output_directory);
        let vendor_js_output_path = format!("{}/assets/vendor.js", &output_directory);
        let application_css_output_path = format!("{}/assets/application.css", &output_directory);
        let index_html_output_path = format!("{}/index.html", &output_directory);
        let memserver_output_path = format!("{}/assets/memserver.js", &output_directory);
        let tests_output_path = format!("{}/assets/tests.js", &output_directory);
        let tests_support_js_path = format!("{}/assets/test-support.js", &output_directory);
        let tests_support_css_path = format!("{}/assets/test-support.css", &output_directory);
        let package_json_path = format!("{}/package.json", &output_directory);

        fs::remove_dir_all(&output_directory).unwrap_or_else(|_| {});

        return Ok((
            current_directory, output_directory, application_js_output_path, vendor_js_output_path,
            application_css_output_path, index_html_output_path, memserver_output_path, tests_output_path,
            tests_support_js_path, tests_support_css_path, package_json_path
        ));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        fs::remove_dir_all("tmp")?;
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_all_assets_works() -> Result<(), Box<dyn Error>> {
        let (
            current_directory, output_directory, application_js_output_path, vendor_js_output_path,
            application_css_output_path, index_html_output_path, memserver_output_path, tests_output_path,
            tests_support_js_path, tests_support_css_path, package_json_path
        ) = setup_test()?;
        let config = Config::build(
            json!({ "environment": "production", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );

        fs::create_dir_all(&output_directory);

        assert!(!fs::metadata(&application_js_output_path).is_ok());
        assert!(!fs::metadata(&vendor_js_output_path).is_ok());
        assert!(!fs::metadata(&application_css_output_path).is_ok());
        assert!(!fs::metadata(&memserver_output_path).is_ok());
        assert!(!fs::metadata(&index_html_output_path).is_ok());

        build_all_assets(&config)?;

        assert!(fs::metadata(application_js_output_path).is_ok());
        assert!(fs::metadata(vendor_js_output_path).is_ok());
        assert!(fs::metadata(application_css_output_path).is_ok());
        assert!(fs::metadata(index_html_output_path).is_ok());
        assert!(!fs::metadata(memserver_output_path).is_ok());
        assert!(fs::metadata(tests_output_path).is_ok());
        assert!(fs::metadata(tests_support_js_path).is_ok());
        assert!(fs::metadata(tests_support_css_path).is_ok());
        assert!(fs::metadata(package_json_path).is_ok());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_all_assets_works_when_tmp_folder_does_not_exist() -> Result<(), Box<dyn Error>> {
        let (
            current_directory, output_directory, application_js_output_path, vendor_js_output_path,
            application_css_output_path, index_html_output_path, memserver_output_path, tests_output_path,
            tests_support_js_path, tests_support_css_path, package_json_path
        ) = setup_test()?;
        let config = Config::build(
            json!({
                "environment": "production",
                "modulePrefix": "frontend",
                "APP": {
                    "API_HOST": "http://localhost:3000"
                },
                "fastboot": {
                    "hostWhitelist": [
                        "^localhost:\\d+$"
                    ]
                },
            }),
            HashMap::new(),
            BuildCache::new()
        );

        fs::remove_dir_all(&output_directory).unwrap_or_else(|_| {});

        assert!(!fs::metadata(&application_js_output_path).is_ok());
        assert!(!fs::metadata(&vendor_js_output_path).is_ok());
        assert!(!fs::metadata(&application_css_output_path).is_ok());
        assert!(!fs::metadata(&memserver_output_path).is_ok());
        assert!(!fs::metadata(&index_html_output_path).is_ok());

        build_all_assets(&config)?;

        assert!(fs::metadata(application_js_output_path).is_ok());
        assert!(fs::metadata(vendor_js_output_path).is_ok());
        assert!(fs::metadata(application_css_output_path).is_ok());
        assert!(fs::metadata(index_html_output_path).is_ok());
        assert!(!fs::metadata(memserver_output_path).is_ok());
        assert!(fs::metadata(tests_output_path).is_ok());
        assert!(fs::metadata(tests_support_js_path).is_ok());
        assert!(fs::metadata(tests_support_css_path).is_ok());
        assert!(fs::metadata(package_json_path).is_ok());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_all_assets_with_memserver_works() -> Result<(), Box<dyn Error>> {
        let (
            current_directory, output_directory, application_js_output_path, vendor_js_output_path,
            application_css_output_path, index_html_output_path, memserver_output_path, tests_output_path,
            tests_support_js_path, tests_support_css_path, package_json_path
        ) = setup_test()?;
        let config = Config::build(
            json!({
                "environment": "production",
                "modulePrefix": "frontend",
                "APP": {
                    "API_HOST": "http://localhost:3000"
                },
                "fastboot": {
                    "hostWhitelist": [
                        "^localhost:\\d+$"
                    ]
                },
                "memserver": {
                    "enabled": true
                }
            }),
            HashMap::new(),
            BuildCache::new()
        );

        fs::remove_dir_all(&output_directory).unwrap_or_else(|_| {});

        assert!(!fs::metadata(&application_js_output_path).is_ok());
        assert!(!fs::metadata(&vendor_js_output_path).is_ok());
        assert!(!fs::metadata(&application_css_output_path).is_ok());
        assert!(!fs::metadata(&memserver_output_path).is_ok());
        assert!(!fs::metadata(&index_html_output_path).is_ok());
        assert!(!fs::metadata(&memserver_output_path).is_ok());
        assert!(!fs::metadata(&package_json_path).is_ok());

        build_all_assets(&config)?;

        assert!(fs::metadata(application_js_output_path).is_ok());
        assert!(fs::metadata(vendor_js_output_path).is_ok());
        assert!(fs::metadata(application_css_output_path).is_ok());
        assert!(fs::metadata(index_html_output_path).is_ok());
        assert!(fs::metadata(memserver_output_path).is_ok());
        assert!(fs::metadata(tests_output_path).is_ok());
        assert!(fs::metadata(tests_support_js_path).is_ok());
        assert!(fs::metadata(tests_support_css_path).is_ok());
        assert!(fs::metadata(package_json_path).is_ok());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_all_assets_works_for_testing() -> Result<(), Box<dyn Error>> {
        let (
            current_directory, output_directory, application_js_output_path, vendor_js_output_path,
            application_css_output_path, index_html_output_path, memserver_output_path, tests_output_path,
            tests_support_js_path, tests_support_css_path, package_json_path
        ) = setup_test()?;
        let mut config = Config::build(
            json!({
                "environment": "production",
                "modulePrefix": "frontend",
                "APP": {
                    "API_HOST": "http://localhost:3000"
                },
                "fastboot": {
                    "hostWhitelist": [
                        "^localhost:\\d+$"
                    ]
                },
                "memserver": {
                    "enabled": true
                },
                "documentation": {
                    "enabled": true
                }
            }),
            HashMap::new(),
            BuildCache::new()
        );
        config.cli_arguments.testing = true;

        fs::remove_dir_all(&output_directory).unwrap_or_else(|_| {});

        assert!(!fs::metadata(&application_js_output_path).is_ok());
        assert!(!fs::metadata(&vendor_js_output_path).is_ok());
        assert!(!fs::metadata(&application_css_output_path).is_ok());
        assert!(!fs::metadata(&memserver_output_path).is_ok());
        assert!(!fs::metadata(&index_html_output_path).is_ok());
        assert!(!fs::metadata(&memserver_output_path).is_ok());
        assert!(!fs::metadata(&tests_output_path).is_ok());
        assert!(!fs::metadata(&tests_support_js_path).is_ok());
        assert!(!fs::metadata(&tests_support_css_path).is_ok());
        assert!(!fs::metadata(&package_json_path).is_ok());

        build_all_assets(&config)?;

        assert!(fs::metadata(application_js_output_path).is_ok());
        assert!(fs::metadata(vendor_js_output_path).is_ok());
        assert!(fs::metadata(application_css_output_path).is_ok());
        assert!(fs::metadata(index_html_output_path).is_ok());
        assert!(fs::metadata(memserver_output_path).is_ok());
        assert!(fs::metadata(tests_output_path).is_ok());
        assert!(fs::metadata(tests_support_js_path).is_ok());
        assert!(fs::metadata(tests_support_css_path).is_ok());
        assert!(fs::metadata(package_json_path).is_ok());

        return finalize_test(current_directory);
    }
}
