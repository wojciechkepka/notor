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

#[derive(Debug, Serialize, TemplateOnce)]
#[template(path = "note.stpl")]
pub struct NoteView {
    note: Note,
}

impl Default for NoteView {
    fn default() -> Self {
        NoteView {
            note: Note {
                note_id: 0,
                title: "Error".to_string(),
                content: Some("missing note".to_string()),
            },
        }
    }
}

impl NoteView {
    pub fn new(note: Note) -> Self {
        NoteView { note }
    }
}
