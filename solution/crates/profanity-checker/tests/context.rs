use profanity_checker::ProfanityChecker;
use rstest::rstest;

#[rstest]
#[case::normal("нормальный текст", false)]
#[case::contains_bad_word("Питон - лучший ЯП!", true)]
fn test_context_analysis(#[case] text: &str, #[case] expected: bool) {
    let checker = ProfanityChecker::new().with_context_analysis(true);
    assert_eq!(checker.check(text), expected);
}
