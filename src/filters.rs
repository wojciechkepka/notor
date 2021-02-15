use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct QueryFilter {
    pub limit: Option<i64>,
    pub tag_id: Option<i32>,
}

impl QueryFilter {
    pub fn builder() -> QueryFilterBuilder {
        QueryFilterBuilder::default()
    }
}

#[derive(Default)]
pub struct QueryFilterBuilder {
    pub limit: Option<i64>,
    pub tag_id: Option<i32>,
}

impl QueryFilterBuilder {
    pub fn limit(mut self, l: i64) -> Self {
        self.limit = Some(l);
        self
    }

    pub fn tag(mut self, t: i32) -> Self {
        self.tag_id = Some(t);
        self
    }

    pub fn build(self) -> QueryFilter {
        QueryFilter {
            limit: self.limit,
            tag_id: self.tag_id,
        }
    }
}
