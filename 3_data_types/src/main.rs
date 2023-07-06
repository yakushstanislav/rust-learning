fn types() {
    let a: u8 = 255;
    let b: &str = "This is a string";
    let c: char = '$';
    let d: bool = true;

    println!("Values: {}, {}, {}, {}", a, b, c, d);
}

fn tuples() {
    let a : (i32, i128, f32) = (1, 1000000000000000000, 3.1416);
    let b = (true, false, true);

    let (a1, a2, a3) = a;

    let (b1, b2, b3) = (b.0, b.1, b.2);

    println!("Tuple A: {a1}, {a2}, {a3}");
    println!("Tuple B: {b1}, {b2}, {b3}");
}

fn arrays() {
    let a = [1, 2, 3];

    let a1 = a[0];
    let a2 = a[1];
    let a3 = a[2];

    println!("Array A: {a1}, {a2}, {a3}");

    let b: [char; 3] = ['a', 'b', 'c'];

    let b1 = b[0];
    let b2 = b[1];
    let b3 = b[2];

    println!("Array B: {b1}, {b2}, {b3}");
}

fn main() {
    types();
    tuples();
    arrays();
}
