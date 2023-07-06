fn print(s: &String) {
    println!("{}", s);
}

fn add_copy(s: &mut String) {
    let c = s.clone();

    s.push_str(c.as_str());
}

fn main() {
    let mut s = String::from("This is a string.");

    print(&s);

    add_copy(&mut s);

    print(&s)
}
