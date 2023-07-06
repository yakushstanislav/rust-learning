use std::collections::HashMap;

fn main() {
    let mut v = HashMap::new();

    v.insert("key_1", 100);
    v.insert("key_2", 200);

    v.entry("key_3").or_insert(300);

    for (k, v) in v {
        println!("K: {}, V: {}", k, v);
    }

    let mut r = HashMap::new();

    let s = "This is a string! This is truth!".to_string();

    for word in s.split_whitespace() {
        let v = r.entry(word).or_insert(0);

        *v += 1;
    }

    for (k, v) in r {
        println!("K: {}, V: {}", k, v);
    }
}
