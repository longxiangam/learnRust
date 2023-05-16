pub fn func(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = func(2, 2);
        assert_eq!(result, 4);

    }
}
