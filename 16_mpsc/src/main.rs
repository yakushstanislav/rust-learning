use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let (tx1, tx2) = (tx.clone(), tx.clone());

    let handle1 = thread::spawn(move || {
        for i in 0..5 {
            tx1.send(i).unwrap();
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 0..5 {
            tx2.send(i).unwrap();
        }
    });

    for _ in 0..10 {
        println!("Recv: {:?}", rx.recv().unwrap())
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
}
