#[cfg(test)]
mod token_set_test_suite {
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;
    use sky_syntax::{TokenKind, TokenSet};

    #[rstest]
    #[case::contains(TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]), TokenKind::IDENTIFIER, true)]
    #[case::contains2(TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]), TokenKind::AND_KW, true)]
    #[case::doesnt_contain(TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]), TokenKind::OR_KW, false)]
    fn test_token_set(
        #[case] token_set: TokenSet,
        #[case] token_kind: TokenKind,
        #[case] expected: bool,
    ) {
        assert_eq!(token_set.contains(token_kind.to_syntax()), expected);
    }

    #[rstest]
    #[case::union_disjoint(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::LPAREN, TokenKind::RPAREN]),
        TokenSet::from(vec![
            TokenKind::IDENTIFIER,
            TokenKind::AND_KW,
            TokenKind::LPAREN,
            TokenKind::RPAREN,
        ]),
    )]
    #[case::union_overlapping(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::AND_KW, TokenKind::LPAREN]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW, TokenKind::LPAREN]),
    )]
    #[case::union_empty(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
    )]
    fn test_union(#[case] set1: TokenSet, #[case] set2: TokenSet, #[case] expected: TokenSet) {
        let result = set1.union(set2);
        for kind in expected.kinds() {
            assert!(result.contains(kind));
        }
    }

    #[rstest]
    #[case::intersection_disjoint(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::LPAREN, TokenKind::RPAREN]),
        TokenSet::from(vec![]),
    )]
    #[case::intersection_overlapping(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::AND_KW, TokenKind::LPAREN]),
        TokenSet::from(vec![TokenKind::AND_KW]),
    )]
    #[case::intersection_empty(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![]),
        TokenSet::from(vec![]),
    )]
    fn test_intersection(
        #[case] set1: TokenSet,
        #[case] set2: TokenSet,
        #[case] expected: TokenSet,
    ) {
        let result = set1.intersection(set2);
        for kind in expected.kinds() {
            assert!(result.contains(kind));
        }
    }

    #[rstest]
    #[case::difference_disjoint(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::LPAREN, TokenKind::RPAREN]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
    )]
    #[case::difference_overlapping(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::AND_KW, TokenKind::LPAREN]),
        TokenSet::from(vec![TokenKind::IDENTIFIER]),
    )]
    #[case::difference_empty(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
    )]
    fn test_difference(#[case] set1: TokenSet, #[case] set2: TokenSet, #[case] expected: TokenSet) {
        let result = set1.difference(set2);
        for kind in expected.kinds() {
            assert!(result.contains(kind));
        }
    }

    #[rstest]
    #[case::symmetric_difference_disjoint(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::LPAREN, TokenKind::RPAREN]),
        TokenSet::from(vec![
            TokenKind::IDENTIFIER,
            TokenKind::AND_KW,
            TokenKind::LPAREN,
            TokenKind::RPAREN,
        ]),
    )]
    #[case::symmetric_difference_overlapping(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::AND_KW, TokenKind::LPAREN]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::LPAREN]),
    )]
    #[case::symmetric_difference_empty(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
    )]
    fn test_symmetric_difference(
        #[case] set1: TokenSet,
        #[case] set2: TokenSet,
        #[case] expected: TokenSet,
    ) {
        let result = set1.symmetric_difference(set2);
        for kind in expected.kinds() {
            assert!(result.contains(kind));
        }
    }

    #[rstest]
    #[case::is_subset_disjoint(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::LPAREN, TokenKind::RPAREN]),
        false,
    )]
    #[case::is_subset_overlapping(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::AND_KW, TokenKind::LPAREN]),
        false,
    )]
    #[case::is_subset_equal(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        true,
    )]
    #[case::is_subset_of_superset(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW, TokenKind::LPAREN]),
        true,
    )]
    #[case::is_subset_empty(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![]),
        false,
    )]
    fn test_is_subset(#[case] set1: TokenSet, #[case] set2: TokenSet, #[case] expected: bool) {
        assert_eq!(set1.is_subset(set2), expected);
    }

    #[rstest]
    #[case::is_superset_disjoint(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::LPAREN, TokenKind::RPAREN]),
        false,
    )]
    #[case::is_superset_overlapping(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::AND_KW, TokenKind::LPAREN]),
        false,
    )]
    #[case::is_superset_equal(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        true,
    )]
    #[case::is_superset_of_subset(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW, TokenKind::LPAREN]),
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        true,
    )]
    #[case::is_superset_empty(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![]),
        true,
    )]
    fn test_is_superset(#[case] set1: TokenSet, #[case] set2: TokenSet, #[case] expected: bool) {
        assert_eq!(set1.is_superset(set2), expected);
    }

    #[rstest]
    #[case::is_disjoint_disjoint(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::LPAREN, TokenKind::RPAREN]),
        true,
    )]
    #[case::is_disjoint_overlapping(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![TokenKind::AND_KW, TokenKind::LPAREN]),
        false,
    )]
    #[case::is_disjoint_empty(
        TokenSet::from(vec![TokenKind::IDENTIFIER, TokenKind::AND_KW]),
        TokenSet::from(vec![]),
        true,
    )]
    fn test_is_disjoint(#[case] set1: TokenSet, #[case] set2: TokenSet, #[case] expected: bool) {
        assert_eq!(set1.is_disjoint(set2), expected);
    }
}
