use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("File Not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("Something went wrong");

    println!("With text:\n{}", contents);
}
