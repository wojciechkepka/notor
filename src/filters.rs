use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryFilter {
    pub limit: Option<i64>,
}

impl Default for QueryFilter {
    fn default() -> Self {
        QueryFilter { limit: None }
    }
}
