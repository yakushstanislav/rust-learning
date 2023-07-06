fn main() {
    let n = vec![100, 200, 300, 400, 500, 600, 700];

    let r: i32 = n.iter().map(|v| v * v).sum();

    println!("Result: {r}");
}
