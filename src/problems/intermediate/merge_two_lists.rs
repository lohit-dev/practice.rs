pub fn merge_two_lists(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let mut final_vec = Vec::new();
    let max_length = first.len().max(second.len());

    for i in 0..max_length {
        if i < first.len() {
            final_vec.push(first[i]);
        }

        if i < second.len() {
            final_vec.push(second[i])
        }
    }

    final_vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_two_sorted_lists() {
        assert_eq!(
            merge_two_lists(vec![1, 3, 5], vec![2, 4]),
            vec![1, 2, 3, 4, 5]
        )
    }
}
