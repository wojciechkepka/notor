use super::*;
use crate::models::{Note, Tag};
use crate::web::{html_from, Index, Login, NoteView, TagView, INDEX_SCRIPT, INDEX_STYLE};
use crate::Error;

pub(crate) async fn get_web(username: String, conn: Db) -> Result<impl Reply, Rejection> {
    let _notes = Note::load_notes_with_tags(QueryFilter::default(), username, &conn)
        .await
        .map_err(Error::from)
        .map_err(WebError::from)
        .map_err(reject::custom)?;

    let body = Index::new(_notes);

    let html = html_from(body, "index", INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}

pub(crate) async fn get_web_note(id: i32, _: String, conn: Db) -> Result<impl Reply, Rejection> {
    let note = Note::load(id, &conn)
        .await
        .map_err(Error::from)
        .map_err(WebError::from)
        .map_err(reject::custom)?;

    let page_title = note.title.clone();
    let mut view = NoteView::new(note);
    view.note_tags = Note::tags(id, &conn)
        .await
        .map_err(Error::from)
        .map_err(WebError::from)
        .map_err(reject::custom)?;

    let html = html_from(view, page_title, INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}

pub(crate) async fn get_web_tagview(
    id: i32,
    username: String,
    conn: Db,
) -> Result<impl Reply, Rejection> {
    let notes = Note::load_notes(QueryFilter::builder().tag(id).build(), username, &conn)
        .await
        .map_err(Error::from)
        .map_err(WebError::from)
        .map_err(reject::custom)?;

    let tag = Tag::load(id, &conn)
        .await
        .map_err(Error::from)
        .map_err(WebError::from)
        .map_err(reject::custom)?;

    let page_title = format!("notes with tag `{}`", &tag.name);
    let view = TagView::new(tag, notes);

    let html = html_from(view, page_title, INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}

pub(crate) async fn get_web_login() -> Result<impl Reply, Rejection> {
    let view = Login::new("");

    let html = html_from(view, "Login".to_string(), INDEX_SCRIPT, INDEX_STYLE)?;

    Ok(reply::html(html))
}
