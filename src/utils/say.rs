use std::process::{Command, Stdio};

pub fn speak(text: &str) {
    let text = Command::new("echo")
        .arg(text)
        .stdout(Stdio::piped())
        .spawn()
        .expect("");

    Command::new("festival")
        .args(&["--tts"])
        .stdin(text.stdout.unwrap())
        .spawn()
        .expect("");
}
