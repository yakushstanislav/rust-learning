enum Message {
    Sum { a: i32, b: i32, c: i32 },
    Print { msg: String },
    Exit,
}

fn handle_message(msg: &Message) {
    let v = match msg {
        Message::Sum { a, b, .. } if (a + b) % 2 == 0 => a + b,
        Message::Sum { a, b, .. } if (a + b) % 2 != 0 => a - b,
        Message::Sum { .. } => 0,
        Message::Print { msg } => {
            println!("{msg}");
            0
        }
        Message::Exit => panic!(),
    };

    println!("Value: {:?}", v);
}

fn main() {
    let m1 = Message::Sum { a: 10, b: 25, c: 0 };
    let m2 = Message::Print {
        msg: String::from("Hello world!"),
    };
    let m3 = Message::Exit;

    handle_message(&m1);
    handle_message(&m2);
    handle_message(&m3);
}
