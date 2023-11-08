use axum::response::{IntoResponse, Response};
use hyper::{HeaderMap, StatusCode};

pub struct HtmxError<B>
where
    B: IntoResponse,
{
    pub body: B,
    pub retarget: String,
    pub reswap: String,
}

impl<B> IntoResponse for HtmxError<B>
where
    B: IntoResponse,
{
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert("HX-Retarget", self.retarget.parse().unwrap());
        headers.insert("HX-Reswap", self.reswap.parse().unwrap());
        return (StatusCode::UNPROCESSABLE_ENTITY, headers, self.body).into_response();
    }
}

#[derive(Debug)]
pub enum Error {
    Internal(String),
}
