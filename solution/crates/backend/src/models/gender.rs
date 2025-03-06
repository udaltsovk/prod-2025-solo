use std::fmt;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, PartialEq, Clone, Debug)]
pub enum Gender {
    MALE,
    FEMALE,
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, ToSchema, PartialEq, Clone, Debug)]
pub enum TargetingGender {
    MALE,
    FEMALE,
    ALL,
}
impl fmt::Display for TargetingGender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
