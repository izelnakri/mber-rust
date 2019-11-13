use yansi::Paint;

pub fn run() -> std::io::Result<()> {
    let version = "0.1.7";

    println!(
        "{} mber {}
mber init | new                    # Sets up the initial ember folder structure
mber serve | server                # Starts your ember development server {}
mber build | b                     # Builds your ember application and outputs to /dist folder
mber console | c                   # Boots your ember application with DOM in a node.js repl
mber test | t                      # Runs your ember tests {}
mber generate | g [type] [name]    # Generate ember files for certain abstraction type
mber delete | d [type] [name]      # Remove ember files for certain abstraction type",
        Paint::red("[mber CLI ".to_owned() + &version + &"] Usage:").bold(),
        Paint::yellow("<command (Default: help)>"),
        Paint::green("[alias: \"mber s\"]"),
        Paint::green("(--server to run them in browser)")
    );

    Ok(())
}
