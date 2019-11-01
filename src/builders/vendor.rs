use std::time::Instant;
use std::path::PathBuf;
use std::str::FromStr;
use std::result::Result;
use std::error::Error;
use std::fs;
use yansi::Paint;
use serde_json::{value::Value};
use super::super::utils::{console, file};
use super::super::transpilers::{import_addon_folder_to_amd}; // convert_es_module and import_addon_folder_to_amd
// use super::super::injections; // documentation, memserver, fetch, fastboot
use super::super::types::Config;

// NOTE: has hard dependency on ember-data(when needed) and ember-cli-fastboot
pub fn build(config: Config) -> Result<(String, fs::Metadata), Box<dyn Error>> {
    console::log(format!("{} vendor.js...", Paint::yellow("BUILDING:")));

    let build_start = Instant::now();
    let project_root = &config.project_root.display();
    let output_path = PathBuf::from_str(format!("{}/tmp/assets/vendor.js", &project_root).as_str())?;
    let _should_minify = vec!["production", "demo"].contains(&config.env["environment"].as_str().unwrap());

    let mut content = vec![
        get_right_ember_base_string(&config.env),
        if &config.env["excludeEmberData"] == true {
            String::from("")
        } else {
            import_addon_folder_to_amd::to_string("ember-data/app", &config)
        },
        String::from_utf8(include_bytes!("../../_vendor/mber-documentation/index.js").to_vec())?
    ].join("\n");

    if config.cli_arguments.fastboot {
        // TODO: add below to content.push_st()
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

    // NOTE: then linting

    return Ok((message, output_metadata));
}

fn get_right_ember_base_string(env: &Value) -> String {
    match (env["excludeEmberData"].as_bool().unwrap(), vec!["production", "demo"].contains(&env["environment"].as_str().unwrap())) {
        (true, true) => String::from_utf8(include_bytes!("../../_vendor/full-ember-prod.js").to_vec()).unwrap(),
        (true, false) => String::from_utf8(include_bytes!("../../_vendor/full-ember-debug.js").to_vec()).unwrap(),
        (false, true) => String::from_utf8(include_bytes!("../../_vendor/no-ember-data-ember-prod.js").to_vec()).unwrap(),
        (false, false) => String::from_utf8(include_bytes!("../../_vendor/no-ember-data-ember-debug.js").to_vec()).unwrap()
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
// development, and dev without ember-data
// fastboot: false works, without ember-data fastboot: false works
// do it for production
// also makes one for custom env
// do appends, prepends and both for development
