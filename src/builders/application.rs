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
pub fn build(config: &Config, _lint: bool) -> Result<(String, fs::Metadata), Box<dyn Error>> {
    console::log(format!("{} application.js...", Paint::yellow("BUILDING:")));

    let build_start = Instant::now();

    let project_root = &config.project_root.display();
    let environment = config.env["environment"].as_str().unwrap_or("development");
    let output_path = PathBuf::from_str(format!("{}/tmp/assets/application.js", &project_root).as_str())?;
    let application_path = PathBuf::from_str(format!("{}/src", &project_root).as_str())?;
    let should_minify = vec!["production", "demo"].contains(&environment);
    let contents = recursive_file_lookup::lookup_for_extensions_and_predicate(
        &application_path,
        vec![".js", ".ts", ".hbs"],
        |entry| { return !entry.file_name().to_str().unwrap().ends_with("-test.js"); }
    ).into_iter()
    .map(|file| transpilers::convert_es_module::from_file(&file, should_minify))
    .collect::<Vec<&str>>()
    .join("\n");
    let application_name = &config.application_name;
    let stringified_env = &config.env.to_string();
    let code = format!("
        {}
        define = window.define;
        {}
        define('{}/config/environment', ['exports'], function (exports) {{
          'use strict';

          exports.__esModule = true;

          if (window.location && (window.location.pathname === '/tests')) {{
            var ENV = Object.assign(JSON.parse({}), {{
              locationType: 'none',
            }});
            ENV.APP = Object.assign(ENV.APP, {{
              autoboot: false,
              rootElement: '#ember-testing'
            }});

            exports.default = ENV;
          }} else {{
            exports.default = JSON.parse({});
          }}

          if (typeof FastBoot !== 'undefined') {{
            return FastBoot.config('{}');
          }}
        }});

        if (typeof FastBoot !== 'undefined') {{
          define('~fastboot/app-factory', ['{}/src/main', '{}/config/environment'], function(App, config) {{
            App = App['default'];
            config = config['default'];

            return {{
              'default': function() {{
                return App.create(config.APP);
              }}
            }};
          }});
        }}

        if (typeof FastBoot === 'undefined' && !runningTests) {{
          require('{}/src/main')['default'].create(require('{}/config/environment').default);
        }}

        {}
    ", config.build_cache.application_prepends, contents, application_name, stringified_env, stringified_env,
    application_name, application_name, application_name, application_name, application_name,
    config.build_cache.application_appends);

    fs::write(&output_path, code)?;

    // TODO: in future create a thread global build error to say/stop tts on error

    let output_metadata = fs::metadata(output_path)?;
    let message = format!(
        "{} application.js in {} [{}] Environment: {}",
        Paint::green("BUILT:"),
        Paint::yellow(file::format_time_passed(build_start.elapsed().as_millis())),
        file::format_size(output_metadata.len()),
        &environment
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

    const APPLICATION_JS_BUILD_TIME_THRESHOLD: u32 = 2000;
    const APPLICATION_JS_TARGET_BYTE_SIZE: u64 = 1100;
    const APPLICATION_JS_COMPRESSED_TARGET_BYTE_SIZE: u64 = 1100;
    const CODE_TO_PREPEND: &str = "(function() { console.log('this is prepending code') })()";
    const CODE_TO_APPEND: &str = "(function() { console.log('this is appending code') })()";

    fn setup_test() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let application_js_output_path = format!("{}/tmp/assets/application.js", &project_directory);

        Paint::disable();
        fs::remove_file(&application_js_output_path).unwrap_or_else(|_| {});
        env::set_current_dir(&project_directory)?;
        fs::create_dir_all("tmp/assets").unwrap_or_else(|_| {});

        return Ok((current_directory, application_js_output_path, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        Paint::enable();
        fs::remove_dir_all("tmp").unwrap_or_else(|_| {});
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_works_for_development() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "moduleprefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(&config, false)?; // note: config and lint
        let build_time_in_ms = Regex::new(r"application\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < APPLICATION_JS_BUILD_TIME_THRESHOLD);

        // let application_js_code = fs::read_to_string(application_js_output_path).unwrap();
        // todo: content checks

        assert!(fs::metadata(application_js_output_path)?.len() >= APPLICATION_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: application\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_production() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "production", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(&config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"application\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < APPLICATION_JS_BUILD_TIME_THRESHOLD);

        // let application_js_code = fs::read_to_string(application_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(application_js_output_path)?.len() >= APPLICATION_JS_COMPRESSED_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: application\.js in \d+ms \[\d+.\d+ kB\] Environment: production")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_custom_environment() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "custom", "modulePrefix": "my-app" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, _stats) = build(&config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"application\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;

        assert!(build_time_in_ms < APPLICATION_JS_BUILD_TIME_THRESHOLD);

        // let application_js_code = fs::read_to_string(application_js_output_path).unwrap();
        // TODO: content checks

        assert!(fs::metadata(application_js_output_path)?.len() >= APPLICATION_JS_TARGET_BYTE_SIZE - 1000);
        assert!(Regex::new(r"BUILT: application\.js in \d+ms \[\d+.\d+ kB\] Environment: custom")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_application_prepends() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new().insert("application_prepends", CODE_TO_PREPEND)
        );
        let (message, _stats) = build(&config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"application\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let application_js_code = fs::read_to_string(&application_js_output_path)?;

        assert!(build_time_in_ms < APPLICATION_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(application_js_output_path)?.len() >= APPLICATION_JS_TARGET_BYTE_SIZE - 1000);
        assert!(application_js_code.contains(CODE_TO_PREPEND));
        assert!(!application_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: application\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_application_appends() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new().insert("application_appends", CODE_TO_APPEND)
        );
        let (message, _stats) = build(&config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"application\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let application_js_code = fs::read_to_string(&application_js_output_path)?;

        assert!(build_time_in_ms < APPLICATION_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(application_js_output_path)?.len() >= APPLICATION_JS_TARGET_BYTE_SIZE - 1000);
        assert!(!application_js_code.contains(CODE_TO_PREPEND));
        assert!(application_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: application\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_with_application_appends_and_prepends() -> Result<(), Box<dyn Error>> {
        let (current_directory, application_js_output_path, _) = setup_test()?;

        assert_eq!(fs::metadata(&application_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
                .insert("application_prepends", CODE_TO_PREPEND)
                .insert("application_appends", CODE_TO_APPEND)
        );
        let (message, _stats) = build(&config, false)?; // NOTE: config and lint
        let build_time_in_ms = Regex::new(r"application\.js in \d+ms")?
            .find(message.as_str()).unwrap().as_str()
            .replace("application.js in ", "")
            .replace("ms", "")
            .parse::<u32>()?;
        let application_js_code = fs::read_to_string(&application_js_output_path)?;

        assert!(build_time_in_ms < APPLICATION_JS_BUILD_TIME_THRESHOLD);
        assert!(fs::metadata(application_js_output_path)?.len() >= APPLICATION_JS_TARGET_BYTE_SIZE - 1000);
        assert!(application_js_code.contains(CODE_TO_PREPEND));
        assert!(application_js_code.contains(CODE_TO_APPEND));
        assert!(Regex::new(r"BUILT: application\.js in \d+ms \[\d+.\d+ kB\] Environment: development")?.find(&message).is_some());

        return finalize_test(current_directory);
    }
}

// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/main'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/resolver'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/router'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/data/models/application/adapter'));
// t.true(
//   codeIncludesAMDModule(applicationJSCode, 'frontend/src/data/models/application/serializer')
// );
// t.true(
//   codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/components/welcome-page/component')
// );
// t.true(
//   codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/components/welcome-page/template')
// );
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/routes/application/head'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/routes/application/route'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/routes/index/route'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/routes/index/template'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/routes/not-found/route'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/routes/not-found/template'));
// t.true(codeIncludesAMDModule(applicationJSCode, 'frontend/config/environment'));
// t.true(
//   applicationJSCode.includes(`if (typeof FastBoot !== 'undefined') {
//         define('~fastboot/app-factory', ['frontend/src/main', 'frontend/config/environment'], function(App, config) {
//           App = App['default'];
//           config = config['default'];

//           return {
//             'default': function() {
//               return App.create(config.APP);
//             }
//           };
//         });
//       }

//       if (typeof FastBoot === 'undefined' && !runningTests) {
//         require('frontend/src/main')['default'].create(require('frontend/config/environment').default);
//       }`)
// );
// t.true(
//   !codeIncludesAMDModule(
//     applicationJSCode,
//     'frontend/src/ui/components/welcome-page/integration-test'
//   )
// );
// t.true(!codeIncludesAMDModule(applicationJSCode, 'frontend/src/ui/routes/index/unit-test'));
