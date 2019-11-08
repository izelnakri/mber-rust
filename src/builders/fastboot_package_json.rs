use std::path::PathBuf;
use std::str::FromStr;
use std::fs;
use std::result::Result;
use std::error::Error;
use serde_json;
use serde_json::{json, Value};
use super::super::types::Config;

pub fn build(asset_map: Value, config: Config, dist_folder: Option<&str>) -> Result<(String), Box<dyn Error>> {
    let target_dist_folder = dist_folder.unwrap_or("dist");
    let target_dist_path = PathBuf::from_str(format!("{}/{}/package.json", &config.project_root.display(), target_dist_folder).as_str())?;
    let application_path = &asset_map["assets/application.js"];
    let application_name = config.application_name;

    let mut original_env = config.env.clone();
    let env = original_env.as_object_mut().unwrap();
    let target_app = env.get_mut("APP").unwrap().as_object_mut().unwrap();

    target_app.insert(String::from_str("autoboot")?, Value::Bool(false));
    target_app.insert(String::from_str("name")?, Value::String(config.env["modulePrefix"].as_str().unwrap().to_string()));
    target_app.insert(String::from_str("version")?, Value::String("0.0.0+b5f80b0d".to_string()));

    let final_app = serde_json::to_value(target_app)?;

    env.insert(String::from_str("APP")?, final_app);
    env.insert(String::from_str("exportApplicationGlobal")?, Value::Bool(true));
    env.insert(String::from_str("isModuleUnification")?, Value::Bool(true));

    let host_whitelist = config.env["fastboot"]["hostWhitelist"].as_array().unwrap();
    let json = json!({
        "dependencies": {},
        "fastboot": {
          "appName": application_name,
          "config": {
              application_name: env
          },
          "hostWhitelist": if host_whitelist.len() == 0 {
              Value::Array(Vec::new())
          } else {
              Value::Array(host_whitelist.to_vec())
          },
          "manifest": {
            "appFiles": if config.env["memserver"]["enabled"].as_bool().unwrap_or(false) {
                vec![application_path, &asset_map["assets/memserver.js"]]
            } else {
                vec![application_path]
            },
            "htmlFile": "index.html",
            "vendorFiles": [asset_map["assets/vendor.js"].as_str().unwrap()]
          },
          "moduleWhitelist": ["node-fetch", "abortcontroller-polyfill"],
          "schemaVersion": 3
        }
    });
    let json_string = serde_json::to_string_pretty(&json)?;

    fs::write(target_dist_path, &json_string)?;

    return Ok(json_string);
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use std::path::PathBuf;
    use serde_json::json;
    use std::collections::HashMap;
    use super::super::super::types::BuildCache;

    fn get_development_env() -> Value {
        return json!({
            "ember-resolver": {
                "features": {
                    "EMBER_RESOLVER_MODULE_UNIFICATION": true
                }
            },
            "modulePrefix": "dummyapp",
            "environment": "development",
            "rootURL": "/",
            "locationType": "auto",
            "fastboot": {
                "hostWhitelist": [
                    "^localhost:\\d+$"
                ]
            },
            "ember-devtools": {
                "global": true,
                "enabled": true
            },
            "memserver": {
                "minify": false,
                "enabled": false
            },
            "EmberENV": {
                "FEATURES": {
                    "ember-module-unification": true
                },
                "EXTEND_PROTOTYPES": {
                    "Date": false
                }
            },
            "APP": {
                "API_HOST": "http://localhost:3000"
            }
        });
    }

    fn get_production_env() -> Value {
        return json!({
            "ember-resolver": {
                "features": {
                    "EMBER_RESOLVER_MODULE_UNIFICATION": true
                }
            },
            "modulePrefix": "dummyapp",
            "environment": "production",
            "rootURL": "/",
            "locationType": "auto",
            "fastboot": {
                "hostWhitelist": [
                    "^localhost:\\d+$"
                ]
            },
            "ember-devtools": {
                "global": true,
                "enabled": false
            },
            "memserver": {
                "minify": true,
                "enabled": false
            },
            "EmberENV": {
                "FEATURES": {
                    "ember-module-unification": true
                },
                "EXTEND_PROTOTYPES": {
                    "Date": false
                }
            },
            "APP": {
                "API_HOST": "http://localhost:3000"
            }
        });
    }

    fn setup_test() -> Result<(PathBuf, Value, Value), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());

        env::set_current_dir(&project_directory)?;
        fs::remove_dir_all("tmp")?;
        fs::remove_dir_all("dist")?;
        fs::create_dir_all("tmp/assets")?; // NOTE: very important breaks other tests otherwise
        fs::create_dir_all("dist")?;

        let example_asset_map: Value = json!({
            "assets/application.js": "assets/application-df0b6cbf528e46c0aa02b74f24252ffd.js",
            "assets/vendor.js": "assets/vendor-339579265dd86542580d6f7cc296dac7.js",
            "assets/memserver.js": "assets/memserver-zaza79265dd86542580d6f7cc296dac7"
        });
        let second_example_asset_map: Value = json!({
            "assets/application.js": "assets/application-aaaa6cbf528e46c0aa02b74f24252ffd.js",
            "assets/vendor.js": "assets/vendor-aaaa79265dd86542580d6f7cc296dac7.js"
        });

        return Ok((current_directory, example_asset_map, second_example_asset_map));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_works_for_and_asset_map_and_env() -> Result<(), Box<dyn Error>> {
        let (current_directory, example_asset_map, _) = setup_test()?;
        let app_files = example_asset_map.get("assets/application.js").unwrap().clone();
        let vendor_files = example_asset_map.get("assets/vendor.js").unwrap().clone();
        let development_env = get_development_env();
        let module_prefix = development_env.get("modulePrefix").unwrap().clone();

        assert_eq!(fs::metadata("tmp/package.json").is_ok(), false);

        let config = Config::build(
            development_env,
            HashMap::new(),
            BuildCache::new()
        );

        build(example_asset_map, config, Some("tmp"))?;

        let package_json: Value =
            serde_json::from_str(fs::read_to_string("tmp/package.json")?.as_str())?;

        assert_eq!(package_json["dependencies"], json!({}));
        assert_eq!(package_json["fastboot"]["appName"].as_str().unwrap(), "dummyapp");
        assert_eq!(package_json["fastboot"]["config"]["dummyapp"]["APP"], json!({
            "API_HOST":  "http://localhost:3000",
            "autoboot": false,
            "name": module_prefix,
            "version": "0.0.0+b5f80b0d"
        }));
        assert_eq!(package_json["fastboot"]["manifest"], json!({
            "appFiles": vec![app_files],
            "htmlFile": "index.html",
            "vendorFiles": vec![vendor_files]
        }));
        assert_eq!(package_json["fastboot"]["hostWhitelist"].as_array().unwrap(), &vec![
            Value::String("^localhost:\\d+$".to_string())
        ]);
        assert_eq!(package_json["fastboot"]["moduleWhitelist"].as_array().unwrap(), &vec![
            Value::String("node-fetch".to_string()), Value::String("abortcontroller-polyfill".to_string())
        ]);
        assert_eq!(package_json["fastboot"]["schemaVersion"].as_u64().unwrap(), 3);

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_different_dist_path_asset_map_and_env() -> Result<(), Box<dyn Error>> {
        let (current_directory, _, second_example_asset_map) = setup_test()?;
        let app_files = second_example_asset_map.get("assets/application.js").unwrap().clone();
        let vendor_files = second_example_asset_map.get("assets/vendor.js").unwrap().clone();
        let production_env = get_production_env();
        let module_prefix = production_env.get("modulePrefix").unwrap().clone();

        assert_eq!(fs::metadata("dist/package.json").is_ok(), false);

        let config = Config::build(
            production_env,
            HashMap::new(),
            BuildCache::new()
        );

        build(second_example_asset_map, config, Some("dist"))?;

        let package_json: Value =
            serde_json::from_str(fs::read_to_string("dist/package.json")?.as_str())?;

        assert_eq!(package_json["dependencies"], json!({}));
        assert_eq!(package_json["fastboot"]["appName"].as_str().unwrap(), "dummyapp");
        assert_eq!(package_json["fastboot"]["config"]["dummyapp"]["APP"], json!({
            "API_HOST":  "http://localhost:3000",
            "autoboot": false,
            "name": module_prefix,
            "version": "0.0.0+b5f80b0d"
        }));
        assert_eq!(package_json["fastboot"]["manifest"], json!({
            "appFiles": vec![app_files],
            "htmlFile": "index.html",
            "vendorFiles": vec![vendor_files]
        }));
        assert_eq!(package_json["fastboot"]["hostWhitelist"].as_array().unwrap(), &vec![Value::String("^localhost:\\d+$".to_string())]);
        assert_eq!(package_json["fastboot"]["moduleWhitelist"].as_array().unwrap(), &vec![
            Value::String("node-fetch".to_string()), Value::String("abortcontroller-polyfill".to_string())
        ]);
        assert_eq!(package_json["fastboot"]["schemaVersion"].as_u64().unwrap(), 3);

        return finalize_test(current_directory);
    }

    #[test]
    fn build_appends_memserver_path_only_on_memserver_mode() -> Result<(), Box<dyn Error>> {
        let (current_directory, example_asset_map, _) = setup_test()?;
        let app_files = example_asset_map.get("assets/application.js").unwrap().clone();
        let vendor_files = example_asset_map.get("assets/vendor.js").unwrap().clone();
        let memserver_files = example_asset_map.get("assets/memserver.js").unwrap().clone();
        let development_env = json!({
            "ember-resolver": {
                "features": {
                    "EMBER_RESOLVER_MODULE_UNIFICATION": true
                }
            },
            "modulePrefix": "dummyapp",
            "environment": "development",
            "rootURL": "/",
            "locationType": "auto",
            "fastboot": {
                "hostWhitelist": [
                    "^localhost:\\d+$"
                ]
            },
            "ember-devtools": {
                "global": true,
                "enabled": true
            },
            "memserver": {
                "minify": false,
                "enabled": true
            },
            "EmberENV": {
                "FEATURES": {
                    "ember-module-unification": true
                },
                "EXTEND_PROTOTYPES": {
                    "Date": false
                }
            },
            "APP": {
                "API_HOST": "http://localhost:3000"
            }
        });
        let module_prefix = development_env.get("modulePrefix").unwrap().clone();

        assert_eq!(fs::metadata("tmp/package.json").is_ok(), false);

        let config = Config::build(
            development_env,
            HashMap::new(),
            BuildCache::new()
        );

        build(example_asset_map, config, Some("tmp"))?;

        let package_json: Value =
            serde_json::from_str(fs::read_to_string("tmp/package.json")?.as_str())?;

        assert_eq!(package_json["dependencies"], json!({}));
        assert_eq!(package_json["fastboot"]["appName"].as_str().unwrap(), "dummyapp");
        assert_eq!(package_json["fastboot"]["config"]["dummyapp"]["APP"], json!({
            "API_HOST":  "http://localhost:3000",
            "autoboot": false,
            "name": module_prefix,
            "version": "0.0.0+b5f80b0d"
        }));
        assert_eq!(package_json["fastboot"]["manifest"], json!({
            "appFiles": vec![app_files, memserver_files],
            "htmlFile": "index.html",
            "vendorFiles": vec![vendor_files]
        }));
        assert_eq!(package_json["fastboot"]["hostWhitelist"].as_array().unwrap(), &vec![Value::String("^localhost:\\d+$".to_string())]);
        assert_eq!(package_json["fastboot"]["moduleWhitelist"].as_array().unwrap(), &vec![
            Value::String("node-fetch".to_string()), Value::String("abortcontroller-polyfill".to_string())
        ]);
        assert_eq!(package_json["fastboot"]["schemaVersion"].as_u64().unwrap(), 3);

        return finalize_test(current_directory);
    }
}
