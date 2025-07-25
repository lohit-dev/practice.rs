pub fn reverse_words(word: &str) -> String {
    let mut rev_char = String::new();
    let chars: Vec<char> = word.chars().collect();

    for i in (0..chars.len()).rev() {
        rev_char.push(chars[i]);
    }

    rev_char
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("abc"), "cba")
    }
}
