use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop value");
    }
}

fn main() {
    let a = Box::new(10);
    let b = MyBox::new(20);

    {
        let _ = MyBox::new(30);
    }

    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("B(Deref): {:?}", *b);
}
