#[warn(dead_code)]
fn simple_panic() {
    panic!("This is a panic!");
}

#[warn(dead_code)]
fn get_error() -> Result<bool, bool> {
    Err(true)
}

fn main() {
    //simple_panic();

    //get_error().unwrap();

    get_error().expect("return error");
}
