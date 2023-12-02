#[cfg(test)]
mod tests {
    use rust_test::functionality::add_two_numbers;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add_two_numbers(3, 5), 8);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add_two_numbers(-2, -7), -9);
    }
}