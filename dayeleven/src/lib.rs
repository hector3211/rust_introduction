#[cfg(test)]
mod tests {
    // when testing we always use
    // #[test] before the function name
    #[test]
    fn it_works() {
        let result = 2 + 2;
        // assert_eq! is a macro that checks if the result is equal to 4
        assert_eq!(result, 4);
    }

    #[test]
    fn failing_test() {
        let result = 2 + 2;
        assert_eq!(result, 5);
    }
}
