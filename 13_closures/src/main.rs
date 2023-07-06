use std::{thread, time::Duration};

fn main() {
    let sum = |v1, v2| v1 + v2;

    println!("Sum: {}", sum(100, 128));

    let v1 = vec![100, 200, 300];

    thread::spawn(move || println!("Vector: {:?}", v1))
        .join()
        .unwrap();

    thread::sleep(Duration::from_secs(1));
}
