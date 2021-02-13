use super::*;
use crate::models::Note;
use crate::html::HtmlContext;
use crate::web::Index;

pub(crate) async fn get_web(conn: Db) -> Result<impl Reply, Rejection> {
    use schema::notes::dsl::*;
    let conn = conn.lock().map_err(|e| DbError::reject(e))?;

    let _notes = notes.load::<Note>(&*conn).map_err(|_| NotFound::reject())?;

    let body = Index::new(_notes);

    let html = HtmlContext::builder()
        .lang("en")
        .title("Notor - index")
        .add_meta("viewport", "width=device-width, initial-scale=1")
        .body(body)
        .build()
        .map_err(|e| InternalError::reject(e))?
        .as_html()
        .map_err(|e| InternalError::reject(e))?;

    Ok(reply::html(html))
}
