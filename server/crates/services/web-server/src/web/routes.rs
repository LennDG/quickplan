use crate::web::{Error, Result};
use axum::extract::State;
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_html::landing_page;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use super::mw_ctx_resolver::CtxW;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/", get(landing_page_handler))
        .with_state(mm)
}

// region:	  --- Landing Page
async fn landing_page_handler(
    State(mm): State<ModelManager>,
    ctx: Result<CtxW>,
) -> Result<Response> {
    debug!("{:<12} - landing_page_handler", "HANDLER");

    Ok(landing_page()?)
}
// endregion: --- Landing Page

// region:	  --- Plan Page
async fn plan_page_handler(State(mm): State<ModelManager>, ctx: CtxW) -> Result<Response> {
    debug!("{:<12} - landing_page_handler", "HANDLER");

    Ok(landing_page()?)
}
// endregion: --- Plan Page
