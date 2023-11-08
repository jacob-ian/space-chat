use askama::Template;
use axum::{body::StreamBody, extract::Path, response::IntoResponse, routing::get, Router};
use include_dir::{include_dir, Dir};
use tokio_util::io::ReaderStream;

use crate::errors::HtmxError;

static ASSETS: Dir<'_> = include_dir!("assets");

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: String,
}

pub fn router() -> Router {
    return Router::new().route("/*path", get(serve_asset));
}

async fn serve_asset(
    Path(path): Path<String>,
) -> Result<impl IntoResponse, HtmxError<ErrorTemplate>> {
    let file = ASSETS.get_file(path).ok_or(HtmxError {
        body: ErrorTemplate {
            message: "File not found".to_string(),
        },
        retarget: "body".to_string(),
        reswap: "innerHTML".to_string(),
    })?;
    let stream = ReaderStream::new(file.contents());
    let body = StreamBody::new(stream);
    return Ok(body);
}
