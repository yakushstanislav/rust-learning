fn get_biggest_value<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }

    b
}

fn make_static() -> &'static str {
    return "Static string!";
}

fn main() {
    let a = make_static();

    let c;

    {
        let b = "Hello world!";

        c = get_biggest_value(a, b);
    }

    println!("{}", c);
}
