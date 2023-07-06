#[derive(Debug)]
enum Position {
    Earth { x: u32, y: u32 },
    Space(),
}

impl Position {
    fn show(&self) {
        println!("{:?}", self)
    }
}

fn display_position(p: &Position) {
    match p {
        Position::Earth { x, y } => println!("You are on Earth ({}, {})", x, y),
        Position::Space() => println!("You are in space!"),
    }
}

fn main() {
    let p1 = Position::Earth { x: 115, y: 230 };

    p1.show();

    let p2 = Position::Space();

    p2.show();

    display_position(&p1);
    display_position(&p2);

    if let Position::Space() = p2 {
        println!("Wow! You are in space!")
    } else {
        println!("Are you on Earth?")
    }
}
