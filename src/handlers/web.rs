use super::*;
use crate::html::HtmlContext;
use crate::models::Note;
use crate::web::{Index, NoteView, INDEX_SCRIPT, INDEX_STYLE};

pub(crate) async fn get_web(conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    let _notes = notes.load::<Note>(&*conn).map_err(|_| NotFound::reject())?;

    let body = Index::new(_notes);

    let html = HtmlContext::builder()
        .lang("en")
        .title("Notor - index")
        .add_meta("viewport", "width=device-width, initial-scale=1")
        .add_script(std::str::from_utf8(INDEX_SCRIPT).map_err(InternalError::reject)?)
        .add_style(std::str::from_utf8(INDEX_STYLE).map_err(InternalError::reject)?)
        .body(body)
        .build()
        .map_err(InternalError::reject)?
        .as_html()
        .map_err(InternalError::reject)?;

    Ok(reply::html(html))
}

pub(crate) async fn get_web_note(id: i32, conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    let note = notes
        .filter(note_id.eq(id))
        .first::<Note>(&*conn)
        .map_err(|_| NotFound::reject())?;

    let page_title = format!("Notor - `{}`", &note.title);
    let view = NoteView::new(note);

    let html = HtmlContext::builder()
        .lang("en")
        .title(page_title)
        .add_meta("viewport", "width=device-width, initial-scale=1")
        .add_script(std::str::from_utf8(INDEX_SCRIPT).map_err(InternalError::reject)?)
        .add_style(std::str::from_utf8(INDEX_STYLE).map_err(InternalError::reject)?)
        .body(view)
        .build()
        .map_err(InternalError::reject)?
        .as_html()
        .map_err(InternalError::reject)?;

    Ok(reply::html(html))
}
