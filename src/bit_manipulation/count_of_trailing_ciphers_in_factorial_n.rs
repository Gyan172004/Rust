/// Function to count the number of trailing ciphers in the factorial of n.
/// 
fn number_of_ciphers_in_factorial_n(mut n: u64) -> u64 {
    // Initialize a variable to count the trailing ciphers.
    let mut count = 0;

    // Keep dividing n by powers of 5 to find the number of trailing ciphers.
    while n >= 5 {
        n /= 5;
        count += n;
    }

    // Return the count of trailing ciphers.
    count
}

/// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(number_of_ciphers_in_factorial_n(395), 97);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(number_of_ciphers_in_factorial_n(977), 242);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(number_of_ciphers_in_factorial_n(871), 215);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(number_of_ciphers_in_factorial_n(239), 57);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(number_of_ciphers_in_factorial_n(0), 0);
    }

    #[test]
    fn test_case_6() {
        assert_eq!(number_of_ciphers_in_factorial_n(25), 6);
    }

    #[test]
    fn test_case_7() {
        assert_eq!(number_of_ciphers_in_factorial_n(7), 1);
    }

    #[test]
    fn test_case_8() {
        assert_eq!(number_of_ciphers_in_factorial_n(1000), 249);
    }
}
