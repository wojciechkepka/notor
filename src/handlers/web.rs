use super::*;
use crate::html::HtmlContext;
use crate::models::Note;
use crate::web::{Index, NoteView, INDEX_SCRIPT, INDEX_STYLE};
use sailfish::TemplateOnce;

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
        .body(body)
        .build()
        .map_err(RejectError::from)
        .map_err(reject::custom)?
        .as_html()
        .map_err(RejectError::from)
        .map_err(reject::custom)
}

pub(crate) async fn get_web(conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    let _notes = Note::load_notes(QueryFilter::default(), &conn)
        .map_err(RejectError::from)
        .map_err(reject::custom)?;

    let body = Index::new(_notes);

    let html = html_from(body, "index", INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}

pub(crate) async fn get_web_note(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    let conn = lock_db(&conn)?;

    let note = Note::load(id, &conn)
        .map_err(RejectError::from)
        .map_err(reject::custom)?;

    let page_title = note.title.clone();
    let view = NoteView::new(note);

    let html = html_from(view, page_title, INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}
