use axum::response::Response;
use axum::routing::get;
use axum::Router;
use lib_core::model::ModelManager;
use lib_html::{about_page, home_page};

use tracing::debug;

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
