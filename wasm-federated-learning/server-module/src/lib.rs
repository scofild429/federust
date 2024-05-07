pub fn serveradd(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = serveradd(2, 2);
        assert_eq!(result, 4);
    }
}
