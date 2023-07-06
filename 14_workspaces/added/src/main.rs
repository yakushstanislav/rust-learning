use sum;

fn main() {
    let mut args = std::env::args();

    args.next();

    let v1 = match args.next() {
        Some(v) => v,
        None => panic!("no value 1"),
    };

    let v2 = match args.next() {
        Some(v) => v,
        None => panic!("no value 2"),
    };

    let v1: u32 = v1.parse().expect("cannot parse value 1");
    let v2: u32 = v2.parse().expect("cannot parse value 2");

    let r = sum::add(v1, v2);

    println!("Sum: {r}");
}
