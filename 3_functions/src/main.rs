fn say_hello(name: &str) {
    println!("Hello, {name}!");
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn expressions() {
    let a = {
        let mut a = 10;

        a = a + 123;

        a
    };

    println!("Expession result: {a}");
}

fn main() {
    say_hello("Stanislav");

    expressions();

    let result = sum(100, 530);

    println!("Sum of two numbers: {result}");
}
