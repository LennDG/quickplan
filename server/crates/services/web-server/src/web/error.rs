use crate::web;
use ::serde::Serialize;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use derive_more::From;
use lib_core::model;
use std::sync::Arc;
use tower_cookies::cookie::time::serde;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
pub enum Error {
    // -- Middleware
    ReqStampNotInReqExt,
    MinificationFailed,

    // -- Modules
    #[from]
    Html(lib_html::Error),

    #[from]
    Model(model::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse

// region:	  --- Client Error
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use web::Error::*;

        match self {
            // -- Fallback.
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

    SERVICE_ERROR,
}
// endregion: --- Client Error

// impl From<lib_html::Error> for Error {
//     fn from(value: lib_html::Error) -> Self {
//         Self::Html(value)
//     }
// }
