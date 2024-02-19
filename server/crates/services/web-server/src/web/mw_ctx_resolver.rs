use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{FromRequestParts, Path, State},
    http::{request::Parts, Request, Uri},
    middleware::Next,
    response::Response,
};
use lib_core::{
    ctx::Ctx,
    model::{plan::PlanBmc, ModelManager},
};
use serde::Serialize;
use tracing::debug;

pub async fn mw_ctx_resolver(
    State(mm): State<ModelManager>,
    path: Path<Vec<(String, String)>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = ctx_resolve(mm, path).await;

    // Store the ctx_ext_result in the request extension
    // (for Ctx extractor).
    req.extensions_mut().insert(ctx_ext_result);

    next.run(req).await
}

async fn ctx_resolve(mm: ModelManager, Path(params): Path<Vec<(String, String)>>) -> CtxExtResult {
    // -- Get the URL ID
    for (a, b) in params {
        debug!("{:<12} - plan_page_handler - {a} - {b}", "HANDLER");
    }
    let url_id = "aaa";

    // TODO: Doing this early prevents unnecessary DB access
    // -- Check if it is a valid format

    // TODO: Do we just stick the plan into the Ctx at this point? Only need a single DB access like that.
    // -- Check if the url_id is in the database
    let plan = PlanBmc::get_plan_by_url(&Ctx::root_ctx(), url_id, &mm)
        .await
        .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
        .ok_or(CtxExtError::UrlIdNotFound)?;

    debug!("{:<12} - ctx_resolve -", "MIDDLEWARE");

    // -- Create CtxExtResult
    Ctx::new(url_id)
        .map(CtxW)
        .map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

// region:    --- Ctx Extractor
#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    UrlIdWrongFormat,
    UrlIdNotFound,

    ModelAccessError(String),

    CtxNotInRequestExt,
    CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
