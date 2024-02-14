// region:	  --- Modules
mod config;
mod error;
mod log;
mod web;

use axum::{middleware, Router};
use lib_core::{_dev_utils, model::ModelManager};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::web::{
    mw_ctx_resolver::mw_ctx_resolver, mw_req_stamp::mw_req_stamp_resolver,
    mw_res_map::mw_response_map, routes::routes,
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

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    let routes_all = Router::new()
        .merge(routes(mm.clone()))
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolver))
        .layer(middleware::from_fn(mw_req_stamp_resolver));

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
