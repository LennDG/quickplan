// region:	  --- Modules
mod config;
mod error;
mod log;
mod web;

use lib_core::_dev_utils;
use tracing_subscriber::EnvFilter;

pub use self::error::{Error, Result};
use config::web_config;

use crate::web::mw_req_stamp::mw_req_stamp_resolver;
// endregion: --- Modules

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;
}
