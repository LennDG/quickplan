// region:	  --- Modules

mod error;
pub mod mw_ctx_resolver;
pub mod mw_html_strip;
pub mod mw_req_stamp;
pub mod mw_res_map;
pub mod plan_routes;
pub mod routes;
pub mod routes_static;
pub mod user_routes;

pub use self::error::ClientError;
pub use self::error::{Error, Result};
// endregion: --- Modules
