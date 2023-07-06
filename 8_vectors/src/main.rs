fn main() {
    let mut v1: Vec<u32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(10);
    v1.push(20);

    for v in &mut v1 {
        *v *= 2;

        println!("Value(1): {}", v);
    }

    for v in &v2 {
        println!("Value(2): {}", v);
    }

    println!("Value(1) - {}", &v1[0]);

    // Return Option<T>
    println!("Value(2) - {:?}", v2.get(0));
    println!("Value(3) - {:?}", v2.get(100));
}
