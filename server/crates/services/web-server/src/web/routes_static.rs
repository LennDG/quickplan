use crate::web_config;
use axum::extract::OriginalUri;
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{any_service, MethodRouter};
use tower_http::services::ServeDir;
use tracing::debug;

// Note: Here we can just return a MethodRouter rather than a full Router
//       since ServeDir is a service.
pub fn not_found() -> MethodRouter {
    any_service(
        ServeDir::new(&web_config().WEB_FOLDER).not_found_service(not_found_handler.into_service()),
    )
}

pub async fn not_found_handler(uri: Uri) -> Response {
    debug!("{:<12} - not_found_handler - {uri}", "HANDLER");

    (StatusCode::NOT_FOUND, lib_html::page_not_found()).into_response()
}
