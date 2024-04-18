use crate::web::routes_static::{not_found, not_found_handler};
use crate::web::{Error, Result};
use axum::extract::{Path, State};
use axum::http::Uri;
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use lib_core::ctx::Ctx;
use lib_core::model::plan::PlanBmc;
use lib_core::model::ModelManager;
use lib_html::plan_template::plan_page;
use lib_html::{about_page, home_page};
use lib_utils::time;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use super::mw_ctx_resolver::CtxW;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/", get(home_page_handler))
        .route("/about", get(about_page_handler))
        .with_state(mm)
}

// region:	  --- Landing Page
async fn home_page_handler() -> Response {
    debug!("{:<12} - home_page_handler", "HANDLER");

    home_page()
}
// endregion: --- Landing Page

// region:	  --- About Page
async fn about_page_handler() -> Response {
    debug!("{:<12} - about_page_handler", "HANDLER");

    about_page()
}
// endregion: --- About Page

