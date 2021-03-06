use std::io;
use std::fs;

use super::super::helpers::mber;

fn setup() -> io::Result<()> {
    if fs::metadata("dummyapp").is_ok() {
        fs::remove_dir_all("dummyapp")?;
    }

    mber::spawn("new dummyapp");

    Ok(())
}

#[test]
fn delete_command_shows_right_exceptions() -> io::Result<()> {
    setup()?;

    let (stdout, output) = mber::spawn("delete");

    assert_eq!(output.status.success(), false);
    assert_eq!(
        stdout.contains("mber d missing an ember abstraction to delete!"),
        true
    );

    let (stdout, output) = mber::spawn("d mdl");

    assert_eq!(output.status.success(), false);
    assert_eq!(
        stdout.contains("mdl is not a valid ember abstraction to delete. Choose one of these abstractions:"),
        true
    );
    assert_eq!(
        stdout.contains("[\"component\", \"helper\", \"initializer\", \"instance-initializer\", \"mixin\", \"model\", \"route\", \"service\", \"util\"]"),
        true
    );

    let (stdout, output) = mber::spawn("d model");

    assert_eq!(output.status.success(), false);
    assert_eq!(
        stdout.contains("mber d model missing a name to delete!"),
        true
    );

    // let (stdout, output) = mber::spawn("d model cities");

    // assert_eq!(output.status.success(), true);
    // assert_eq!(
        // stdout.contains(),
        // true
    // );
    // TODO: also create the respective file
    // TODO: then make one that passes

    fs::remove_dir_all("dummyapp")?;

    Ok(())
}
