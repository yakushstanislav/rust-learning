pub fn add(v1: u32, v2: u32) -> u32 {
    v1 + v2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
