// region:	  --- Modules
mod config;
mod error;
mod log;
mod web;

use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    Router,
};
use lib_core::{_dev_utils, model::ModelManager};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::web::{
    mw_ctx_resolver::mw_ctx_resolver, mw_html_strip::mw_html_strip,
    mw_req_stamp::mw_req_stamp_resolver, mw_res_map::mw_response_map, routes, routes_static,
};

pub use self::error::{Error, Result};
use config::web_config;
// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // -- DEV ONLY
    mm.dev_seed().await;

    // -- Define Routes
    let routes_all = Router::new()
        .merge(web::routes::routes(mm.clone()))
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn(mw_req_stamp_resolver))
        .fallback_service(routes_static::not_found())
        .layer(tower_http::compression::CompressionLayer::new().gzip(true));

    // region:    --- Start Server
    // Note: For this block, ok to unwrap.
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}
