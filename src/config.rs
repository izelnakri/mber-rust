use std::collections::HashMap
use std::path::Path
use hyper::Uri

struct CLIArguments {
    env: str,
    port: u16,
    proxy: Uri,
    server: bool,
    fastboot: bool,
    watch: bool,
    debug: bool,
    talk: bool,
    testing: bool // NOTE: is this necessary?
}

struct BuildCache {
    vendor_appends: str,
    vendor_prepends: str,
    application_appends: str,
    application_prepends: str,
    test_appends: str,
    test_prepends: str,
}

struct Config {
    application_name: str,
    build_cache: BuildCache,
    cli_arguments: CLIArguments,
    ENV: HashMap,
    index_html_injections: Vec,
    project_root: Path
}
