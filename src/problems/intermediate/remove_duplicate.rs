use std::collections::HashSet;

pub fn remove_duplicates(list: Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::new();
    let mut vec = Vec::new();

    for value in list {
        if set.insert(value) {
            vec.push(value);
        }
    }

    vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        assert_eq!(
            remove_duplicates(vec![1, 2, 1, 1, 2, 3, 5]),
            vec![1, 2, 3, 5]
        );
        assert_eq!(
            remove_duplicates(vec![1, 2, 7, 7, 3, 1, 2, 3, 5]),
            vec![1, 2, 7, 3, 5]
        )
    }
}
