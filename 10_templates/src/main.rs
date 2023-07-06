#[derive(Debug)]
enum Status<T, E> {
    Success(T),
    Err(E),
}

struct Point<T: std::fmt::Debug> {
    x: T,
    y: T,
}

impl<T: std::fmt::Debug> Point<T> {
    fn print(&self) {
        println!("X: {:?}, Y: {:?}", self.x, self.y);
    }
}

impl Point<&str> {
    fn is_string(&self) {
        println!("This is string, dude!");
    }
}

fn max<T: std::cmp::PartialOrd>(v1: T, v2: T) -> T {
    if v1 > v2 {
        return v1;
    }

    v2
}

fn main() {
    println!("Max: {}", max(100, 1024));

    let p = Point { x: 10.0, y: 20.0 };
    p.print();

    let p = Point { x: "10", y: "20" };
    p.is_string();

    let s: Status<&str, bool> = Status::Success("success");
    println!("Status: {:?}", s);
}
