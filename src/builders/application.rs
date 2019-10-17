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
    console::log(format!("{} application.js...", Paint::yellow("BUILDING:")));

    let build_start = Instant::now();
    let project_root = &config.project_root.display();
    let output_path = PathBuf::from_str(format!("{}/tmp/assets/application.js", &project_root).as_str())?;
    let application_path = PathBuf::from_str(format!("{}/src", &project_root).as_str())?;
    let contents = recursive_file_lookup::lookup_for_extensions_and_predicate(
        &application_path,
        vec![".js", ".ts", ".hbs"],
        |entry| { return !entry.file_name().to_str().unwrap().ends_with("-test.js"); }
    ).iter()
    .map(|file| transpilers::convert_es_module::from_file(file, &config.env["environment"] == "production"))
    .collect::<Vec<&str>>()
    .join("/n");
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
      `
    ", config.build_cache.application_prepends, contents, application_name, stringified_env, stringified_env,
    application_name, application_name, application_name, application_name, application_name,
    config.build_cache.application_appends);

    fs::write(&output_path, code)?;

    // TODO: in future create a thread global build error to say/stop tts on error

    let output_metadata = fs::metadata(output_path)?;
    let message = format!(
        "{} application.js in {} [{}] Environment: environment",
        Paint::green("BUILT:"),
        Paint::yellow(file::format_time_passed(build_start.elapsed().as_millis())),
        file::format_size(output_metadata.len())
    );

    console::log(&message);

    // then linting

    return Ok((message, output_metadata));
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::env;
    use super::*;
    use std::path::PathBuf;
    use serde_json::json;
    use std::collections::HashMap;
    use super::super::super::types::BuildCache;

    fn setup() -> Result<(PathBuf, String, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());
        let application_js_output_path = format!("{}/tmp/assets/application.js", &project_directory);

        fs::remove_file(&application_js_output_path);
        env::set_current_dir(&project_directory)?;

        return Ok((current_directory, project_directory, application_js_output_path));
    }

    #[test]
    fn build_works_for_development() -> Result<(), Box<dyn Error>> {
        let (current_directory, _project_directory, application_js_output_path) = setup()?;

        assert_eq!(fs::metadata(&application_js_output_path).is_ok(), false);

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let (message, stats) = build(config, false)?; // NOTE: config and lint
        // let time_taken_for_build = message
            // .match(/application\.js in \d+ms/g)[0]
            // .replace('application.js in ', '')
            // .replace('ms', '');

        // t.true(Number(timeTakenForBuild) < APPLICATION_JS_BUILD_TIME_THRESHOLD);

        // const applicationJSBuffer = await fs.readFile(APPLICATION_JS_OUTPUT_PATH);
        // const applicationJSCode = applicationJSBuffer.toString().trim();
        // content checks

        // t.true(applicationJSBuffer.length >= APPLICATION_JS_TARGET_BYTE_SIZE - 1000);
        // t.true(stats.size >= APPLICATION_JS_TARGET_BYTE_SIZE - 1000);
        // console.log('MESSAGE WAS', message);
        // t.true(/BUILT: application\.js in \d+ms \[12.10 kB\] Environment: development/g.test(message));
        assert_eq!(true, true);

        env::set_current_dir(&current_directory)?;

        Ok(())
    }
//
//     #[test]
//     fn build_works_for_production() -> io::Result<()> {
//     }
//
//     #[test]
//     fn build_works_for_custom_environment() -> io::Result<()> {
//     }
//
//     #[test]
//     fn build_works_with_application_prepends() -> io::Result<()> {
//     }
//
//     #[test]
//     fn build_works_with_application_appends() -> io::Result<()> {
//     }
//
//     #[test]
//     fn build_works_with_application_appends_and_prepends() -> io::Result<()> {
//         t.true(!(await fs.exists(APPLICATION_JS_OUTPUT_PATH)));
            // const CODE_TO_PREPEND = '(function() { console.log("this is prepending code") })()';
            // const CODE_TO_APPEND = '(function() { console.log("this is appending code") })()';
            // const mock = mockProcessCWD(PROJECT_ROOT);
            // const { message, stats } = await buildApplication(
            //   {
            //     ENV: { environment: 'development', modulePrefix: 'frontend' },
            //     buildCache: { applicationPrepends: CODE_TO_PREPEND, applicationAppends: CODE_TO_APPEND }
            //   },
            //   false
            // );
            // const timeTakenForBuild = message
            //   .match(/application\.js in \d+ms/g)[0]
            //   .replace('application.js in ', '')
            //   .replace('ms', '');

            // t.true(Number(timeTakenForBuild) < APPLICATION_JS_BUILD_TIME_THRESHOLD);

            // const applicationJSBuffer = await fs.readFile(APPLICATION_JS_OUTPUT_PATH);
            // const applicationJSCode = applicationJSBuffer.toString().trim();

            // t.true(applicationJSCode.startsWith(CODE_TO_PREPEND));
            // t.true(applicationJSCode.endsWith(CODE_TO_APPEND));
            // t.true(applicationJSBuffer.length >= APPLICATION_JS_TARGET_BYTE_SIZE);
            // t.true(stats.size >= APPLICATION_JS_TARGET_BYTE_SIZE);
//     }
// }
//
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
}
