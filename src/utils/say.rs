use std::process::{Command, Stdio};

pub fn speak(text: &str) {
    if tts_program_in_path("festival") {
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
    } else {
        println!("\007");
    }
}

fn tts_program_in_path(program: &str) -> bool {
    match Command::new(program).stdout(Stdio::null()).spawn() {
        Ok(_) => true,
        Err(_) => false
    }
}

