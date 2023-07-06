#[warn(dead_code)]
pub fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let s = sum(1, 3);

        assert_eq!(s, 4);
    }

    #[test]
    #[ignore]
    fn invalid_test_sum() {
        let s = sum(1, 4);

        assert_eq!(s, 10);
    }

    #[test]
    #[should_panic(expected = "This is a sum")]
    fn panic_test() {
        let s = sum(1, 4);

        panic!("This is a sum: {}", s);
    }
}
