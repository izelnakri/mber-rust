// use hyper::Uri;

// #[derive(Debug)]
// enum Proxy {
//     Uri, None
// }

#[derive(Debug)]
pub struct CLIArguments {
    env: &'static str,
    port: u16,
    // proxy: Option<Proxy>,
    server: bool,
    fastboot: bool,
    watch: bool,
    debug: bool,
    talk: bool,
    testing: bool // NOTE: is this necessary?
}

impl CLIArguments {
    pub fn parse() -> Self { // TODO: add different arguments, overwritable
        CLIArguments {
            env: "development",
            port: 1234,
            // proxy: None,
            server: true,
            fastboot: true,
            watch: true,
            debug: false,
            talk: true,
            testing: false
        }
    }
}
