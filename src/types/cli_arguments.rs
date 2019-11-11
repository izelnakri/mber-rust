// use hyper::Uri;

// #[derive(Debug)]
// enum Proxy {
//     Uri, None
// }

#[derive(Debug)]
pub struct CLIArguments {
    pub env: &'static str,
    pub port: u16,
    // proxy: Option<Proxy>,
    pub server: bool,
    pub fastboot: bool,
    pub watch: bool,
    pub debug: bool,
    pub talk: bool,
    pub testing: bool // NOTE: is this necessary?
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
            testing: true
        }
    }
}
