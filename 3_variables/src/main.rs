const GLOBAL_CONST: i32 = 30;

fn variables1() {
    let mut a = 10;

    println!("Before change: {}", a);

    a = 20;

    println!("After change: {}", a);
}

fn variables2() {
    let c = 30;

    println!("Value 1: {c}");

    let c = "This is a string";

    println!("Value 2: {c}");

    {
        let c = 0.5;

        println!("Value 3: {c}");
    }

    println!("Value 4: {c}");
}

fn constants() {
    const LOCAL_CONST: i32 = 20;

    println!("Local const value {LOCAL_CONST}");
    println!("Global const value {GLOBAL_CONST}");
}

fn main() {
    variables1();
    variables2();
    constants();
}