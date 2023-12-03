#[cfg(test)]
mod tests {
    use day01_05_rust::day0_0::find_first_digit;
    use day01_05_rust::day0_0::find_last_digit;
    use day01_05_rust::day0_0::combine_digits;

    #[test]
    fn test_find_first_digit() {
        assert_eq!(find_first_digit("abc123"), Some(1));
        assert_eq!(find_first_digit("no digits"), None);
        assert_eq!(find_first_digit("987xyz"), Some(9));
        assert_eq!(find_first_digit("0 starts"), Some(0));
        assert_eq!(find_first_digit("x7y8"), Some(7));
        assert_eq!(find_first_digit(""), None);
    }

    #[test]
    fn test_find_last_digit() {
        assert_eq!(find_last_digit("abc123"), Some(3));
        assert_eq!(find_last_digit("no digits"), None);
        assert_eq!(find_last_digit("123xyz"), Some(3));
        assert_eq!(find_last_digit("ends with 0"), Some(0));
        assert_eq!(find_last_digit("x7y8z"), Some(8));
        assert_eq!(find_last_digit(""), None);
    }

    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_digits() {
        assert_eq!(combine_digits(1, 2), 12);
        assert_eq!(combine_digits(0, 9), 9);
        assert_eq!(combine_digits(9, 0), 90);
        assert_eq!(combine_digits(5, 5), 55);
        assert_eq!(combine_digits(0, 0), 0);
    }
}

}
