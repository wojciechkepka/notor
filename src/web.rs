use crate::models::Note;
use sailfish::TemplateOnce;
use serde::Serialize;

#[derive(Default, Debug, Serialize, TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    notes: Vec<Note>,
}

impl Index {
    pub fn new(notes: Vec<Note>) -> Self {
        Index { notes }
    }
}
