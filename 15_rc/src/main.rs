use std::rc::Rc;

fn print_rc_count(a: &Rc<i32>) {
    println!("RC count: {}", Rc::strong_count(a));
}

fn main() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);

    print_rc_count(&b);
}
