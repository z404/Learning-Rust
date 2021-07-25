use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let mut file = File::open("info.txt").expect("UHH i cant open that file..");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("I cant read that!");

    println!("File:\n{}", content);
}
