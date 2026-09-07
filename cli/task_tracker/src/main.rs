mod commands;

use std::fs::File;
use std::io::ErrorKind;

#[allow(unused_variables)] // cant go in production;
fn main() {
    let file = match File::open("TODO") {
        Ok(fc) => fc,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::open("TODO").expect("File doesnt exist and cant create."),
            _ => panic!("An error occured.")
        }
    };
}
