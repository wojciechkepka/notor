use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct QueryFilter {
    pub limit: Option<i64>,
}
