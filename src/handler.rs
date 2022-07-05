use std::borrow::Cow;

use anyhow::Result;

use hyper::{header::LOCATION, Body, Request, Response};

use crate::preset::Provider;

pub async fn handle(_req: Request<Body>) -> Result<Response<Body>> {
    let mut query = url::form_urlencoded::parse(_req.uri().query().unwrap_or("").as_bytes());
    let query = query
        .find(|(k, _)| k == "q")
        .map_or_else(|| Cow::Borrowed(""), |(_, v)| v);

    let result = Provider::handle(query.as_ref());

    let response = Response::builder()
        .header(LOCATION, result)
        .status(307)
        .body(Body::empty())?;

    Ok(response)
}
