use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let value = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let value = Arc::clone(&value);

        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut v = value.lock().unwrap();

                *v += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Value: {:?}", value);
}
