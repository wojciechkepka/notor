use super::*;
use crate::html::HtmlContext;
use crate::models::Note;
use crate::web::{Index, INDEX_SCRIPT, INDEX_STYLE};

pub(crate) async fn get_web(conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    let _notes = notes.load::<Note>(&*conn).map_err(|_| NotFound::reject())?;

    let body = Index::new(_notes);

    let html = HtmlContext::builder()
        .lang("en")
        .title("Notor - index")
        .add_meta("viewport", "width=device-width, initial-scale=1")
        .add_script(std::str::from_utf8(INDEX_SCRIPT).map_err(|e| InternalError::reject(e))?)
        .add_style(std::str::from_utf8(INDEX_STYLE).map_err(|e| InternalError::reject(e))?)
        .body(body)
        .build()
        .map_err(|e| InternalError::reject(e))?
        .as_html()
        .map_err(|e| InternalError::reject(e))?;

    Ok(reply::html(html))
}
