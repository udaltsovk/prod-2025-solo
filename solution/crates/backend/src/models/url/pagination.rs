use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Deserialize, IntoParams, Validate, Debug)]
pub struct Pagination {
    #[validate(range(min = 0))]
    /// Количество элементов на странице.
    pub size: Option<i64>,

    #[validate(range(min = 0))]
    /// Номер страницы.
    pub page: Option<i64>,
}
