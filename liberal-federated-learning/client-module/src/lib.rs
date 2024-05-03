pub fn clientadd(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = clientadd(2, 2);
        assert_eq!(result, 4);
    }
}
