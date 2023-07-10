use std::cell::RefCell;

trait Bus {
    fn handle(&self, msg: &str);
}

#[derive(Debug)]
struct Test {
    msgs: RefCell<Vec<String>>,
}

impl Test {
    fn new() -> Self {
        Test {
            msgs: RefCell::new(vec![]),
        }
    }
}

impl Bus for Test {
    fn handle(&self, msg: &str) {
        self.msgs.borrow_mut().push(msg.to_string());
    }
}

fn main() {
    let a = Test::new();

    a.handle("Message 1");
    a.handle("Message 2");

    dbg!(a);
}
