pub fn fibonacci(number: i32) -> Vec<i32> {
    let mut sequence = Vec::new();

    if number >= 1 {
        sequence.push(0);
    }

    if number >= 2 {
        sequence.push(1);
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..number {
        let c = a + b;
        sequence.push(c);
        a = b;
        b = c;
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibanocci() {
        assert_eq!(fibonacci(5), [0, 1, 1, 2, 3]);
        assert_eq!(fibonacci(8), [0, 1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(fibonacci(10), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
