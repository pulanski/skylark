mod token_set;
#[cfg(test)]
mod syntax_test_suite {
    use pretty_assertions_sorted::assert_eq;
    use sky_syntax::File;

    #[test]
    fn test_syntax() {
        let source = r"def foo():
    pass
";
        let ast = File::parse(source);
        println!("ast: {:#?}", ast);
        assert_eq!(ast.debug_dump(), source);
    }
}
