use std::env;

pub fn run() -> std::io::Result<()> {
    let application_name = std::env::args()
        .nth(3)
        .expect("You forgot to include an application name! Example: mber init example-app");

    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(())
}
