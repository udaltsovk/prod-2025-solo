use profanity_checker::ProfanityChecker;
use rstest::rstest;

#[rstest]
#[case::swear("питон", true)]
#[case::normal("цветок", false)]
fn test_basic_profanity(#[case] word: &str, #[case] expected: bool) {
    let checker = ProfanityChecker::new().with_context_analysis(false);
    assert_eq!(checker.check(word), expected);
}
