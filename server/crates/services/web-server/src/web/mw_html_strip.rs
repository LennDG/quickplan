use crate::web::{Error, Result};
use axum::{
    body::{self, Body, HttpBody},
    http::{header, HeaderValue},
    response::{Html, IntoResponse, Response},
};
use lib_utils::time::now_utc;
use minify_html_onepass::{in_place, Cfg};
use time::Duration;
use tracing::debug;

const CFG: &Cfg = &Cfg {
    minify_js: false,
    minify_css: false,
};

pub async fn mw_html_strip(res: Response) -> Response {
    let tic = now_utc();

    //-- Only strip text/html content
    if res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .filter(|content_type| *content_type == "text/html; charset=utf-8")
        .is_none()
    {
        return res;
    }

    // -- Skip minimization if the HTML body is too large
    if res
        .body()
        .size_hint()
        .upper()
        .filter(|size_upper_limit| *size_upper_limit < 1_000_000)
        .is_none()
    {
        return res;
    }

    // -- Extract parts and body from response
    let (mut parts, body) = res.into_parts();

    let initial_content_length = parts
        .headers
        .get(header::CONTENT_LENGTH)
        .and_then(|length| length.to_str().ok())
        .and_then(|length| length.parse::<usize>().ok())
        .unwrap_or(1);

    // -- FIXME: 2 unwraps being used here make this pretty unsafe to use.
    // At the moment this middleware is disabled because comments are now removed in build script.

    // Safe to set limit to MAX because it will always be smaller than 1MB dure to check earlier.
    let mut bytes = body::to_bytes(body, usize::MAX).await.unwrap().to_vec();

    in_place(&mut bytes, CFG)
        .map(|length| {
            bytes.truncate(length);
            parts.headers.insert(
                header::CONTENT_LENGTH,
                HeaderValue::from_str(&length.to_string()).unwrap(),
            );
            debug!(
                "{:<12} - mw_html_strip - removed {} bytes - compression ratio {:.2}",
                "MIDDLEWARE",
                initial_content_length - length,
                length as f32 / initial_content_length as f32,
            );
        })
        .map_err(|_| Error::MinificationFailed);

    (parts, bytes).into_response()
}
