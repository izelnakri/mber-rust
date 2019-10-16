use std::path::Path;
use serde_json::value::Value;
use std::collections::HashMap;

pub mod build_cache;
pub mod cli_arguments;

pub use build_cache::BuildCache;
pub use cli_arguments::CLIArguments;

pub struct Config {
    pub application_name: &'static str,
    pub build_cache: Box<BuildCache>,
    pub cli_arguments: Box<CLIArguments>,
    pub ENV: Value,
    pub index_html_injections: HashMap<String, String>,
    pub project_root: Path
}

impl Config {
    fn build() {
        println!("Building configuration...");
    }
}
