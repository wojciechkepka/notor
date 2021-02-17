use super::*;
use crate::html::HtmlContext;
use crate::models::{Note, Tag};
use crate::web::{Index, Login, NoteView, TagView, INDEX_SCRIPT, INDEX_STYLE};
use sailfish::TemplateOnce;

const FONT_AWESOME: &str =
    "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.2/css/all.min.css";

fn html_from<B, T, S>(body: B, title: T, script: S, style: S) -> Result<String, Rejection>
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
                .map_err(RejectError::from)
                .map_err(reject::custom)?,
        )
        .add_style(
            std::str::from_utf8(style.as_ref())
                .map_err(RejectError::from)
                .map_err(reject::custom)?,
        )
        .add_style_src(FONT_AWESOME)
        .body(body)
        .build()
        .map_err(RejectError::from)
        .map_err(reject::custom)?
        .as_html()
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn get_web(username: String, conn: Db) -> Result<impl Reply, Rejection> {
    let _notes = Note::load_notes_with_tags(QueryFilter::default(), &conn)
        .await
        .map_err(RejectError::from)
        .map_err(reject::custom)?;

    let body = Index::new(_notes);

    let html = html_from(body, "index", INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}

pub(crate) async fn get_web_note(
    id: i32,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let note = Note::load(id, &conn)
        .await
        .map_err(RejectError::from)
        .map_err(reject::custom)?;

    let page_title = note.title.clone();
    let mut view = NoteView::new(note);
    view.note_tags = Note::tags(id, &conn)
        .await
        .map_err(RejectError::from)
        .map_err(reject::custom)?;

    let html = html_from(view, page_title, INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}

pub(crate) async fn get_web_tagview(
    id: i32,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let notes = Note::load_notes(QueryFilter::builder().tag(id).build(), &conn)
        .await
        .map_err(RejectError::from)
        .map_err(reject::custom)?;

    let tag = Tag::load(id, &conn)
        .await
        .map_err(RejectError::from)
        .map_err(reject::custom)?;

    let page_title = format!("notes with tag `{}`", &tag.name);
    let view = TagView::new(tag, notes);

    let html = html_from(view, page_title, INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}

pub(crate) async fn get_web_login() -> Result<impl Reply, Rejection> {
    let view = Login {};

    let html = html_from(view, "Login".to_string(), INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}
