const MY_AGE: i32 = 32;

fn describe_my_age(age: i32) {
    println!("You age is {age}!");

    if age <= 3 {
        println!("You are small baby!");
    } else if age <= 45 {
        println!("You are is so young!");
    } else {
        println!("You are so wise!");
    }
}

fn greater_or_less(a: i32, b: i32) {
    let result = if a > b { "greater" } else { "less" };

    println!("{a} {result} {b}");
}

fn print_hello(count: i32) {
    let mut counter = 0;

    loop {
        println!("Hello!");

        counter = counter + 1;

        if counter >= count {
            break;
        }
    }
}

fn pow2v1(number: i32) -> i32 {
    let mut c = 0;
    let mut tmp = 2;

    return loop {
        tmp = tmp * 2;
        c = c + 1;

        if tmp >= number {
            break c - 1;
        }
    };
}

fn pow2v2(number: i32) -> i32 {
    let mut c = 0;
    let mut tmp = 2;

    while tmp < number {
        tmp = tmp * 2;
        c = c + 1;
    }

    return c - 1;
}

fn print_numbers(numbers: [i32; 10]) {
    for value in numbers {
        println!("Value: {}", value);
    }
}

fn print_range(start: i32, end: i32) {
    let range = start..end;

    for value in range {
        println!("Value: {}", value);
    }
}

fn main() {
    describe_my_age(MY_AGE);

    greater_or_less(100, 200);
    greater_or_less(30, 10);

    print_hello(2);

    let result: i32 = pow2v1(4096);
    println!("pow2v1(4096): {result}");

    let result: i32 = pow2v2(4096);
    println!("pow2v2(4096): {result}");

    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_numbers(numbers);

    print_range(10, 25);
}
