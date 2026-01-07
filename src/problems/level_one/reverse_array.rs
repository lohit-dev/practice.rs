pub fn reverse_array(mut array: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let mut right = array.len().saturating_sub(1);

    while left < right {
        array.swap(left, right);
        //let temp = array[left];
        //array[left] = array[right];
        //array[right] = temp;

        left += 1;
        right -= 1;
    }

    array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_array() {
        assert_eq!(reverse_array(vec![1, 2, 3, 4]), vec![4, 3, 2, 1]);
        assert_eq!(reverse_array(vec![1, 2, 3]), vec![3, 2, 1]);
        assert_eq!(reverse_array(vec![42]), vec![42]);
        assert_eq!(reverse_array(vec![]), vec![]);
        assert_eq!(reverse_array(vec![10, 20]), vec![20, 10]);
        assert_eq!(reverse_array(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
        assert_eq!(reverse_array(vec![7, 7, 7, 7]), vec![7, 7, 7, 7]);
        assert_eq!(reverse_array(vec![-1, -2, -3]), vec![-3, -2, -1]);
        assert_eq!(reverse_array(vec![-10, 0, 5, -3]), vec![-3, 5, 0, -10]);
        assert_eq!(reverse_array(vec![1, 2, 2, 3]), vec![3, 2, 2, 1]);
        assert_eq!(
            reverse_array(vec![i32::MIN, 0, i32::MAX]),
            vec![i32::MAX, 0, i32::MIN]
        );
    }
}
