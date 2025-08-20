pub fn add_one(left: u64, right: u64) -> u64 {
    left + right + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2, 2);
        assert_eq!(result, 5);
    }
}
