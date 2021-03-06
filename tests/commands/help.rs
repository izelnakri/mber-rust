use super::super::helpers::mber;

#[test]
fn help_command_works() {
    let (stdout, output) = mber::spawn("help");

    assert_eq!(output.status.success(), true);
    assert_eq!(
        stdout.contains("unknown command. Available options are:"),
        false
    );

    [
        "Usage: mber <command (Default: help)>",
        "mber init | new                    # Sets up the initial ember folder structure",
        "mber serve | server                # Starts your ember development server [alias: \"mber s\"]",
        "mber build | b                     # Builds your ember application and outputs to /dist folder",
        "mber console | c                   # Boots your ember application with DOM in a node.js repl",
        "mber test | t                      # Runs your ember tests (--server to run them in browser)",
        "mber generate | g [type] [name]    # Generate ember files for certain abstraction type",
        "mber delete | d [type] [name]      # Remove ember files for certain abstraction type"
    ].iter().for_each(|string| {
        assert_eq!(stdout.contains(string), true);
    })
}
