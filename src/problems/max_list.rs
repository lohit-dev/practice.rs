pub fn max_list(list: Vec<i32>) -> i32 {
    let mut max = list[0];
    (0..list.len()).for_each(|i| {
        if list[i] > max {
            max = list[i]
        }
    });

    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_list() {
        assert_eq!(max_list(vec![1, 6, 3, 4, 2]), 6);
        assert_eq!(max_list(vec![1, 8, 23, 2]), 23)
    }
}
