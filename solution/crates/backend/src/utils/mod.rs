use std::fmt::Debug;

pub mod logger;
pub mod minio;
pub mod openapi;
pub mod validation;

pub trait RoundToDigits: PartialEq + Debug {
    fn round_to_digits(&self, digits: i32) -> Self;
}

impl RoundToDigits for f32 {
    fn round_to_digits(&self, digits: i32) -> Self {
        let factor = 10.0_f32.powi(digits);
        (self * factor).round() / factor
    }
}

impl RoundToDigits for f64 {
    fn round_to_digits(&self, digits: i32) -> Self {
        let factor = 10.0_f64.powi(digits);
        (self * factor).round() / factor
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::RoundToDigits;

    #[rstest]
    #[case(10.0 / 3.0, 2, 3.33)]
    fn round_to_digits<T: RoundToDigits>(
        #[case] input: T,
        #[case] digits: i32,
        #[case] expected: T,
    ) {
        assert_eq!(input.round_to_digits(digits), expected);
    }
}
