use std::path::Path;
use serde_json::value::Value;

pub mod build_cache;
pub mod cli_arguments;

pub use build_cache::BuildCache;
pub use cli_arguments::CLIArguments;

pub struct Config {
    application_name: &'static str,
    build_cache: Box<BuildCache>,
    cli_arguments: Box<CLIArguments>,
    ENV: Value,
    index_html_injections: Vec<String>,
    project_root: Path
}

impl Config {
    fn build() {
        println!("Building configuration...");
    }
}
