// TODO: use tokio
// TODO: make passable new command tests single-threaded
use super::super::helpers::mber;
use std::env;
use std::fs;
use std::io;

fn setup() -> io::Result<()> {
    if fs::metadata("dummyapp").is_ok() {
        fs::remove_dir_all("dummyapp")?;
    }

    if fs::metadata("somethingapp").is_ok() {
        fs::remove_dir_all("somethingapp")?;
    }

    Ok(())
}

#[test]
fn new_command_errors_when_no_application_name() {
    let (stdout, output) = mber::spawn("new");

    assert_eq!(output.status.success(), false);
    assert_eq!(
        stdout
            .contains("You forgot to include an application name! Example: mber init example-app"),
        true
    );
}

#[test]
fn new_command_gives_error_when_app_exists() -> io::Result<()> {
    setup()?;
    fs::create_dir_all("dummyapp")?;

    let directory = env::current_dir()?;
    let repo_name = directory.file_name().unwrap().to_str().unwrap();
    let (stdout, output) = mber::spawn(format!("new {}", repo_name).as_str());

    assert_eq!(output.status.success(), false);
    assert_eq!(
        stdout.contains(format!("{} already exists!", repo_name).as_str()),
        true
    );

    let (stdout, output) = mber::spawn("new dummyapp");

    assert_eq!(output.status.success(), false);
    assert_eq!(stdout.contains("dummyapp already exists!"), true);

    let (stdout, output) = mber::spawn("new somethingapp");

    assert_eq!(output.status.success(), true);
    assert_eq!(stdout.contains("ember creating somethingapp"), true);

    [
        ".dockerignore",
        ".editorconfig",
        ".eslintrc.js",
        "config",
        "index.html",
        "package.json",
        "public",
        "src",
        "tests",
        "tmp",
        "vendor",
    ]
    .iter()
    .for_each(|file_or_folder| {
        println!("{}", file_or_folder);
        assert_eq!(
            stdout.contains(format!("created {}", file_or_folder).as_str()),
            true
        );
    });

    assert_eq!(
        stdout.contains("ember somethingapp ember application created. Next is to do:"),
        true
    );
    assert_eq!(
        stdout.contains("$ cd somethingapp && npm install && mber s"),
        true
    );

    let directory_entries: Vec<String> = fs::read_dir("somethingapp")
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();

    [
        ".dockerignore",
        ".editorconfig",
        ".eslintrc.js",
        ".gitignore",
        "config",
        "index.html",
        "package.json",
        "public",
        "src",
        "tests",
        "tmp",
        "vendor",
    ]
    .iter()
    .for_each(|file_or_folder| {
        assert_eq!(
            *&directory_entries
                .iter()
                .any(|entry| entry == file_or_folder),
            true
        )
    });

    fs::remove_dir_all("dummyapp")?;
    fs::remove_dir_all("somethingapp")?;

    Ok(())
}
