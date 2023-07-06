//use std::collections::{self, BTreeMap, HashMap};
//use std::fmt::*;

pub mod system;
fn main() {
    let s = system::Statistics::new(10, 40);

    s.print();

    println!("Status: {:?} ({:?})", s.get_status(), s);

    system::fs::create_file();
    system::fs::delete_file();
    system::shutdown();
}
