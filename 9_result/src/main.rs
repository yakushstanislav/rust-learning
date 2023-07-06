use std::fs::File;
use std::io::ErrorKind;

const FILE_NAME: &str = "hello.txt";

fn main() {
    let file = File::open(FILE_NAME).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(FILE_NAME).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("Metadata: {:?}", file.metadata());
}
