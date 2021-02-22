use crate::html::HtmlContext;
use crate::models::{Note, NoteWithTags, Tag};
use crate::Error;
use sailfish::TemplateOnce;
use serde::Serialize;
use warp::{reject, Rejection};

pub const INDEX_SCRIPT: &[u8] = include_bytes!("../static/js/glue.js");
pub const INDEX_STYLE: &[u8] = include_bytes!("../static/css/style.css");
pub const FONT_AWESOME: &str =
    "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.2/css/all.min.css";

pub fn html_from<B, T, S>(body: B, title: T, script: S, style: S) -> Result<String, Rejection>
where
    B: TemplateOnce + Default,
    T: AsRef<str>,
    S: AsRef<[u8]>,
{
    HtmlContext::builder()
        .lang("en")
        .title(format!("Notor - {}", title.as_ref()))
        .add_meta("viewport", "width=device-width, initial-scale=1")
        .add_script(
            std::str::from_utf8(script.as_ref())
                .map_err(Error::from)
                .map_err(reject::custom)?,
        )
        .add_style(
            std::str::from_utf8(style.as_ref())
                .map_err(Error::from)
                .map_err(reject::custom)?,
        )
        .add_style_src(FONT_AWESOME)
        .body(body)
        .build()
        .map_err(Error::from)
        .map_err(reject::custom)?
        .as_html()
        .map_err(Error::from)
        .map_err(reject::custom)
}

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
                user_id: 0,
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

#[derive(Default, Debug, Serialize, TemplateOnce)]
#[template(path = "login.stpl")]
pub struct Login {
    err: String,
}

impl Login {
    pub fn new<S: Into<String>>(err: S) -> Self {
        Login { err: err.into() }
    }
}

#[derive(Default, Debug, Serialize, TemplateOnce)]
#[template(path = "404.stpl")]
pub struct NotFound {
    url: String,
}

impl NotFound {
    pub fn new<S: Into<String>>(url: S) -> Self {
        NotFound { url: url.into() }
    }
}
