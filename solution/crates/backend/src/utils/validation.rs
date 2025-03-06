use profanity_checker::ProfanityChecker;
use std::{ops::Not, sync::LazyLock};

use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::{config, routes::ApiError};

static PROFANITY_CHECKER: LazyLock<ProfanityChecker> = LazyLock::new(ProfanityChecker::new);
static PROFANITY_CHECKER_ADVANCED: LazyLock<ProfanityChecker> =
    LazyLock::new(|| ProfanityChecker::new().with_typo_check(1, 4));

pub fn check_profanity(text: &str) -> Result<(), ValidationError> {
    if config::MODERATION_ENABLED.not() {
        return Ok(());
    }

    if PROFANITY_CHECKER.check(text) {
        return Err(ValidationError::new("Found bad words"));
    }

    Ok(())
}

pub fn check_profanity_advanced(text: &str) -> Result<(), ValidationError> {
    if config::MODERATION_ENABLED.not() {
        return Ok(());
    }

    if PROFANITY_CHECKER_ADVANCED.check(text) {
        return Err(ValidationError::new("Found bad words"));
    }

    Ok(())
}

pub fn parse_validation_errors(errors: ValidationErrors) -> ApiError {
    ApiError::InvalidInput(validation_errors_to_string(errors, None))
}

pub fn validation_errors_to_string(errors: ValidationErrors, adder: Option<String>) -> String {
    let mut output = String::new();

    let map = errors.into_errors();

    let key_option = map.keys().next();

    if let Some(field) = key_option {
        if let Some(error) = map.get(field) {
            return match error {
                ValidationErrorsKind::Struct(errors) => {
                    validation_errors_to_string(*errors.clone(), Some(format!("of item {field}")))
                }
                ValidationErrorsKind::List(list) => {
                    if let Some((index, errors)) = list.iter().next() {
                        output.push_str(&validation_errors_to_string(
                            *errors.clone(),
                            Some(format!("of list {field} with index {index}")),
                        ));
                    }

                    output
                }
                ValidationErrorsKind::Field(errors) => {
                    if let Some(error) = errors.first() {
                        if let Some(adder) = adder {
                            output.push_str(&format!(
                                "Field {} {} failed validation with error: {}",
                                field, adder, error.code
                            ));
                        } else {
                            output.push_str(&format!(
                                "Field {} failed validation with error: {}",
                                field, error.code
                            ));
                        }
                    }

                    output
                }
            };
        }
    }

    String::new()
}
