use std::fmt;
use yansi::Paint;

// NOTE: in future turn these to macro?
pub fn log<T: fmt::Display>(item: T) {
    println!("{} {}", Paint::red("ember").bold(), item);
}

pub fn error<T: fmt::Display>(item: T) {
    println!("{} {}", Paint::red("ember").bold(), Paint::red(item));
}
