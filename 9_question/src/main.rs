use std::fs::File;
use std::io::{self, Read};

pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut line = String::new();

    file.read_to_string(&mut line)?;

    Ok(line)
}

fn main() {
    println!("Result (1): {:?}", read_file("not_exists_file.txt"));
    println!("Result (2): {:?}", read_file("hello.txt"));
}
