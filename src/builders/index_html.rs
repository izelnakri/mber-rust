use std::result::Result;
use std::error::{Error};
use std::fs;
use std::collections::HashMap;
use mustache;
use mustache::MapBuilder;
use super::super::types::Config;

pub fn build(html_path: &str, config: &Config) -> Result<String, Box<dyn Error>> {
    let output_path = match html_path.ends_with("tests/index.html") {
        true => format!("{}/tmp/tests.html", &config.project_root.display()),
        false => format!("{}/tmp/index.html", &config.project_root.display())
    };
    let mut content = transpile_mustache_template(html_path, &config.index_html_injections)?;

    if config.env["memserver"]["enabled"].as_bool().unwrap_or(false) {
        content = content.replace(
            "<script src=\"/assets/vendor.js\"></script>",
            "<script src=\"/assets/vendor.js\"></script>
            <script src=\"/assets/memserver.js\"></script>"
        );
    }

    fs::write(output_path, &content)?;

    return Ok(content);
}

pub fn build_documentation_html(html_path: &str, config: &Config) -> Result<String, Box<dyn Error>> {
    let documentation_path_in_config = &config.env["documentation"]["path"].as_str().unwrap_or("/styleguide");
    let output_path = format!("{}/tmp{}.html", &config.project_root.display(), documentation_path_in_config);
    let mut content = transpile_mustache_template(html_path, &config.index_html_injections)?;

    content = content.replace(
        "<link rel=\"stylesheet\" href=\"/assets/application.css\">",
        "<link rel=\"stylesheet\" href=\"/assets/application.css\">\n<link rel=\"stylesheet\" href=\"/assets/documentation.css\">"
    ).replace(
        "<script src=\"/assets/application.js\"></script>",
        "<script src=\"/assets/documentation.js\"></script>\n<script src=\"/assets/application.js\"></script>"
    );

    if config.env["memserver"]["enabled"].as_bool().unwrap_or(false) {
        content = content.replace(
            "<script src=\"/assets/vendor.js\"></script>",
            "<script src=\"/assets/vendor.js\"></script>
            <script src=\"/assets/memserver.js\"></script>"
        );
    }

    fs::write(output_path, &content)?;

    return Ok(content);
}

fn transpile_mustache_template(template_path: &str, index_html_injections: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let dynamic_data = &index_html_injections.into_iter()
        .fold(MapBuilder::new(), |result, (injection_key, injection_value)| {
            return result.insert_str(injection_key, injection_value);
        }).build();
    let template = mustache::compile_path(&template_path)?;

    return Ok(template.render_data_to_string(&dynamic_data)?);
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;
    use std::collections::HashMap;
    use serde_json::json;
    use yansi::Paint;
    use super::*;
    use super::super::super::types::BuildCache;

    fn setup_test() -> Result<(PathBuf, String), Box<dyn Error>> {
        let current_directory = env::current_dir()?;
        let project_directory = format!("{}/ember-app-boilerplate", current_directory.to_string_lossy());

        Paint::disable();
        env::set_current_dir(&project_directory)?;
        fs::create_dir_all("tmp").unwrap_or_else(|_| {});

        return Ok((current_directory, project_directory));
    }

    fn finalize_test(actual_current_directory: PathBuf) -> Result<(), Box<dyn Error>> {
        Paint::enable();
        fs::remove_dir_all("tmp")?;
        env::set_current_dir(&actual_current_directory)?;

        return Ok(());
    }

    #[test]
    fn build_works() -> Result<(), Box<dyn Error>> {
        let (current_directory, project_directory) = setup_test()?;
        let html_input_path = format!("{}/index.html", &project_directory);
        let html_output_path = format!("{}/tmp/index.html", project_directory);

        fs::remove_file(&html_output_path).unwrap_or_else(|_| {});

        assert!(!fs::metadata(&html_output_path).is_ok());

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let output_html = build(&html_input_path.as_str(), &config)?;
        let content = fs::read_to_string(html_output_path)?;

        assert_eq!(output_html, content);
        assert!(output_html.contains("assets/application.css"));
        assert!(output_html.contains("assets/application.js"));
        assert!(output_html.contains("assets/vendor.js"));
        assert!(!output_html.contains("assets/test-support.css"));
        assert!(!output_html.contains("assets/test-support.js"));
        assert!(!output_html.contains("assets/tests.js"));
        assert!(!output_html.contains("assets/memserver.js"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_TITLE -->"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_HEAD -->"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_BODY -->"));

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_different_endpoint() -> Result<(), Box<dyn Error>> {
        let (current_directory, project_directory) = setup_test()?;
        let html_input_path = format!("{}/tests/index.html", &project_directory);
        let html_output_path = format!("{}/tmp/tests.html", project_directory);

        fs::remove_file(&html_output_path).unwrap_or_else(|_| {});

        assert!(!fs::metadata(&html_output_path).is_ok());

        let config = Config::build(
            json!({ "environment": "development", "modulePrefix": "frontend" }),
            HashMap::new(),
            BuildCache::new()
        );
        let output_html = build(&html_input_path.as_str(), &config)?;
        let content = fs::read_to_string(html_output_path)?;

        assert_eq!(output_html, content);
        assert!(output_html.contains("assets/application.css"));
        assert!(output_html.contains("assets/application.css"));
        assert!(output_html.contains("assets/application.js"));
        assert!(output_html.contains("assets/vendor.js"));
        assert!(!output_html.contains("assets/memserver.js"));
        assert!(output_html.contains("assets/test-support.css"));
        assert!(output_html.contains("assets/test-support.js"));
        assert!(output_html.contains("assets/tests.js"));

        assert!(output_html.contains("assets/application.js"));

        return finalize_test(current_directory);
    }

    #[test]
    fn build_works_for_custom_app_with_html_injections_and_memserver() -> Result<(), Box<dyn Error>> {
        let (current_directory, project_directory) = setup_test()?;
        let html_input_path = format!("{}/index.html", &project_directory);
        let html_output_path = format!("{}/tmp/index.html", project_directory);

        fs::remove_file(&html_output_path).unwrap_or_else(|_| {});

        assert!(!fs::metadata(&html_output_path).is_ok());

        let mut index_html_injections: HashMap<String, String> = HashMap::new();

        index_html_injections.insert(
            String::from("googleAnalytics"),
            String::from("<script>console.log('googleAnalytics comes here')</script>")
        );

        let config = Config::build(
            json!({ "environment": "memserver", "modulePrefix": "izelapp", "memserver": { "enabled": true } }),
            index_html_injections,
            BuildCache::new()
        );
        let output_html = build(&html_input_path.as_str(), &config)?;
        let content = fs::read_to_string(html_output_path)?;

        assert_eq!(output_html, content);
        assert!(output_html.contains("assets/application.css"));
        assert!(output_html.contains("assets/application.js"));
        assert!(output_html.contains("assets/vendor.js"));
        assert!(!output_html.contains("assets/test-support.css"));
        assert!(!output_html.contains("assets/test-support.js"));
        assert!(!output_html.contains("assets/tests.js"));
        assert!(output_html.contains("assets/memserver.js"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_TITLE -->"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_HEAD -->"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_BODY -->"));
        assert!(output_html.contains("<script>console.log('googleAnalytics comes here')</script>"));

        return finalize_test(current_directory);
    }

    #[test]
    fn build_documentation_works() -> Result<(), Box<dyn Error>> {
        let (current_directory, project_directory) = setup_test()?;
        let html_input_path = format!("{}/index.html", &project_directory);
        let mut index_html_injections: HashMap<String, String> = HashMap::new();

        index_html_injections.insert(
            String::from("googleAnalytics"),
            String::from("<script>console.log('googleAnalytics comes here')</script>")
        );
        let documentation_path = "/documentation";
        let html_output_path = format!("{}/tmp/{}.html", &project_directory, documentation_path);

        fs::remove_file(&html_output_path).unwrap_or_else(|_| {});

        assert!(!fs::metadata(&html_output_path).is_ok());

        let config = Config::build(json!({
            "environment": "development", "modulePrefix": "frontend",
            "documentation": { "path": documentation_path }
        }), index_html_injections, BuildCache::new());
        let output_html = build_documentation_html(&html_input_path.as_str(), &config)?;
        let content = fs::read_to_string(html_output_path)?;

        assert_eq!(output_html, content);
        assert!(output_html.contains("assets/application.css"));
        assert!(output_html.contains("assets/application.js"));
        assert!(output_html.contains("assets/vendor.js"));
        assert!(!output_html.contains("assets/test-support.css"));
        assert!(!output_html.contains("assets/test-support.js"));
        assert!(!output_html.contains("assets/tests.js"));
        assert!(!output_html.contains("assets/memserver.js"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_TITLE -->"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_HEAD -->"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_BODY -->"));
        assert!(output_html.contains("<script>console.log('googleAnalytics comes here')</script>"));

        return finalize_test(current_directory);
    }

    #[test]
    fn build_documentation_works_for_custom_app_with_html_injections_and_memserver() -> Result<(), Box<dyn Error>> {
        let (current_directory, project_directory) = setup_test()?;
        let html_input_path = format!("{}/index.html", &project_directory);
        let mut index_html_injections: HashMap<String, String> = HashMap::new();

        index_html_injections.insert(
            String::from("googleAnalytics"),
            String::from("<script>console.log('googleAnalytics came here')</script>")
        );
        let documentation_path = "/documentation";
        let html_output_path = format!("{}/tmp/{}.html", &project_directory, documentation_path);

        fs::remove_file(&html_output_path).unwrap_or_else(|_| {});

        assert!(!fs::metadata(&html_output_path).is_ok());

        let config = Config::build(json!({
            "environment": "memserver", "modulePrefix": "custom-app",
            "memserver": { "enabled": true }, "documentation": { "path": documentation_path }
        }), index_html_injections, BuildCache::new());
        let output_html = build_documentation_html(&html_input_path.as_str(), &config)?;
        let content = fs::read_to_string(html_output_path)?;

        assert_eq!(output_html, content);
        assert!(output_html.contains("assets/application.css"));
        assert!(output_html.contains("assets/application.js"));
        assert!(output_html.contains("assets/vendor.js"));
        assert!(!output_html.contains("assets/test-support.css"));
        assert!(!output_html.contains("assets/test-support.js"));
        assert!(!output_html.contains("assets/tests.js"));
        assert!(output_html.contains("assets/memserver.js"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_TITLE -->"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_HEAD -->"));
        assert!(output_html.contains("<!-- EMBER_CLI_FASTBOOT_BODY -->"));
        assert!(output_html.contains("<script>console.log('googleAnalytics came here')</script>"));

        return finalize_test(current_directory);
    }
}
