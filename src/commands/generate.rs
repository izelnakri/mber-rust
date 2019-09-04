pub fn run() {
    println!("run generate command here");
}

// extern crate tokio;

// use tokio::prelude::{AsyncRead, Future};

// let task = tokio::fs::File::open("./Cargo.toml")
//     .and_then(|mut file| {
//         let mut contents = vec![];

//         file.read_buf(&mut contents)
//             .map(|_res| println!("{}", String::from_utf8(contents).unwrap()))
//     }).map_err(|err| eprintln!("IO error: {:?}", err));

// let task = tokio::fs::File::open("./Cargo.toml")
//     .and_then(|mut file| {
//         // do something with the file ...
//         let string = String::new();

//         // file.read_to_string(&mut string);
//         // println!("{}", string);
//         file
//     })
//     .map_err(|e| {
//         // handle errors
//         eprintln!("IO error: {:?}", e);
//     });

// tokio::run(task);
