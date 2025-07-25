pub fn check_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    };

    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_check_prime() {
        assert!(check_prime(2));
        assert!(check_prime(3));
        assert!(!check_prime(4));
        assert!(check_prime(5));
        assert!(!check_prime(1));
        assert!(!check_prime(0));
        assert!(!check_prime(-7));
    }
}
