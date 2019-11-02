use std::path::PathBuf;
use serde_json::{value::Value};
use std::collections::HashMap;
use super::utils;

pub mod build_cache;
pub mod cli_arguments;

pub use build_cache::BuildCache;
pub use cli_arguments::CLIArguments;

#[derive(Debug)]
pub struct Config {
    pub application_name: String,
    pub build_cache: Box<BuildCache>,
    pub cli_arguments: Box<CLIArguments>,
    pub env: Value,
    pub index_html_injections: HashMap<String, String>,
    pub project_root: PathBuf
}

impl Config {
    pub fn build<'a>(env: Value, index_html_injections: HashMap<String, String>, build_cache: BuildCache) -> Config {
        Config {
            application_name: String::from(env["modulePrefix"].as_str().unwrap()),
            build_cache: Box::new(build_cache),
            cli_arguments: Box::new(CLIArguments::parse()),
            env: env,
            index_html_injections: index_html_injections,
            project_root: utils::find_project_root()
        }
    }
}
