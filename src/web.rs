use crate::models::Note;
use sailfish::TemplateOnce;
use serde::Serialize;

pub const INDEX_SCRIPT: &[u8] = include_bytes!("../static/js/glue.js");
pub const INDEX_STYLE: &[u8] = include_bytes!("../static/css/style.css");

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
