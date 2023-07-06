fn main() {
    let str = String::from("Simple string");

    println!("{}", str);

    let mut mstr = String::from("My name is ");

    mstr.push_str("Stanislav");

    println!("{}", mstr.clone());

    // Print string (chars)
    for c in mstr.chars() {
        println!("{}", c);
    }

    // Print string (bytes)
    for b in mstr.bytes() {
        println!("{}", b);
    }
}
