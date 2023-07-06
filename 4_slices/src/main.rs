fn get_slice_range(value: &str, begin: usize, end: usize) -> &str {
    return &value[begin..end];
}

fn main() {
    let value = String::from("This is a string!");

    let v = get_slice_range(value.as_str(), 0, 10);

    println!("{}", v);

    let v = get_slice_range("Simple string", 7, 13);

    println!("{}", v);
}
