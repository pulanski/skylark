mod token_set;
#[cfg(test)]
mod syntax_test_suite {
    use pretty_assertions_sorted::assert_eq;
    use sky_syntax::TokenSet;

    #[test]
    fn test_syntax() {
        let actual = "Hello, world!";
        let expected = "Hello, world!";
        assert_eq!(actual, expected);
    }
}
