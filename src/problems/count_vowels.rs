pub fn count_vowels(word: &str) -> i32 {
    let vowels = "aeiou";
    let mut count = 0;

    let chars: Vec<char> = word.to_lowercase().chars().collect();

    (0..chars.len()).for_each(|i| {
        if vowels.contains(chars[i]) {
            count += 1;
        }
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("revolution"), 5)
    }
}
