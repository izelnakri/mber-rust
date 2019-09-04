struct CLIArguments {
    // TODO: add CLI Arguments
}

struct BuildCache {
    vendor_appends: String,
    vendor_prepends: String,
    application_appends: String,
    application_prepends: String,
    test_appends: String,
    test_prepends: String,
}

struct CLI {
    application_name: String,
    build_cache: BuildCache,
    cli_arguments: CLIArguments,
    ENV: Any,                   // TODO: give a good type
    index_html_injections: Any, // TODO: give a good type
    project_root: String,       // TODO: make a Path type
}
