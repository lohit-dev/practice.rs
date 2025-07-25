pub fn anagram(word: &str, another_word: &str) -> bool {
    let mut w1: Vec<char> = word.chars().collect();
    let mut w2: Vec<char> = another_word.chars().collect();
    w1.sort_unstable();
    w2.sort_unstable();
    w1 == w2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_anagram() {
        assert!(anagram("silent", "listen"));
        assert!(anagram("anagram", "nagaram"));
    }
}
