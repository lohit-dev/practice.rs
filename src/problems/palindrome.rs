pub fn palindrome(name: &str) -> &str {
    let chars: Vec<char> = name.chars().collect();
    //println!("The chars: {chars:?}");

    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() - i - 1] {
            return "not palindrome";
        }
    }
    "palindrome"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert_eq!(palindrome("racecar"), "palindrome");
        assert_eq!(palindrome("madam"), "palindrome");
        assert_eq!(palindrome("hello"), "not palindrome");
    }
}
