pub fn sum_digits(word: &str) -> i32 {
    word.chars()
        .filter(|c| c.is_numeric())
        .map(|c| (c.to_digit(10).unwrap()) as i32)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_digits() {
        assert_eq!(sum_digits("12345"), 15);
    }
}
