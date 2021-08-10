//used for command line arguments
use std::env;
//used for reading and writing into files
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    //creating file with exception error message
    let mut file = File::open("info.txt").expect("UHH i cant open that file..");

    //reading the whole file
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("I cant read that!");

    //printing the file
    println!("File:\n{}", content);

    //taking command line arguements
    let args: Vec<String> = env::args().collect();
    for i in args.iter() {
        println!("{}", i);
    }

    //writing to a file
    let mut file = File::create("output.txt").expect("Couldn't create file");
    file.write_all(b"Welcome to rust!")
        .expect("Couldnt write, sorry :(");

    //switch case statement in rust
    let number = 2;
    match number {
        1 => println!("Hello!"),
        2 => {
            println!("Hello!!");
            println!("Hello again!!");
        }
        10 | 11 => println!("Helo!"),
        _ => println!("Hello!!!"),
    }
}
