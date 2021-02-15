use crate::models::{Note, NoteWithTags, Tag};
use sailfish::TemplateOnce;
use serde::Serialize;

pub const INDEX_SCRIPT: &[u8] = include_bytes!("../static/js/glue.js");
pub const INDEX_STYLE: &[u8] = include_bytes!("../static/css/style.css");

#[derive(Default, Debug, Serialize, TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Index {
    notes: Vec<NoteWithTags>,
}

impl Index {
    pub fn new(notes: Vec<NoteWithTags>) -> Self {
        Index { notes }
    }
}

#[derive(Debug, Serialize, TemplateOnce)]
#[template(path = "note.stpl")]
pub struct NoteView {
    note: Note,
    pub note_tags: Vec<Tag>,
}

impl Default for NoteView {
    fn default() -> Self {
        NoteView {
            note: Note {
                id: 0,
                created: chrono::offset::Utc::now().naive_utc(),
                title: "Error".to_string(),
                content: Some("missing note".to_string()),
            },
            note_tags: vec![],
        }
    }
}

impl NoteView {
    pub fn new(note: Note) -> Self {
        NoteView {
            note,
            note_tags: vec![],
        }
    }
}

#[derive(Default, Debug, Serialize, TemplateOnce)]
#[template(path = "tagview.stpl")]
pub struct TagView {
    tag: Tag,
    notes: Vec<Note>,
}

impl TagView {
    pub fn new(tag: Tag, notes: Vec<Note>) -> Self {
        TagView { tag, notes }
    }
}
