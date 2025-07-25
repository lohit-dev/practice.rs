use std::collections::HashMap;

pub fn matching_brace(word: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let matches = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);

    for v in word.chars() {
        if matches.contains_key(&v) {
            stack.push(v);
        } else if matches.values().any(|&val| val == v) {
            match stack.pop() {
                Some(open) => match matches.get(&open) {
                    Some(&expected) if expected == v => continue,
                    _ => return false,
                },
                None => return false,
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matching_brace() {
        assert!(matching_brace("{([])}"));
        assert!(matching_brace("{([])}"));
        assert!(matching_brace("({[]})"));
        assert!(!matching_brace("([{})]"));
    }
}
