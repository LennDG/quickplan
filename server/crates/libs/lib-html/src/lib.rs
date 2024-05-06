use std::collections::HashMap;

use askama::Template;
use axum::response::{IntoResponse, Response};
use lib_core::model::plan::Plan;
use time::Date;

// region:	  --- Modules
pub mod calendar_utils;
pub mod error;
pub mod plan_template;

pub use self::error::{Error, Result};
// endregion: --- Modules

// region:	  --- Page structs
#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "homepage.html")]
struct HomePageTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate {
    title: String,
}
// endregion: --- Page structs

// region:	  --- Landing page
pub fn home_page() -> Response {
    HomePageTemplate {
        title: "Pick The Day".to_string(),
    }
    .into_response()
}
// endregion: --- Landing page

// region:	  --- About page
pub fn about_page() -> Response {
    AboutTemplate {
        title: "About".to_string(),
    }
    .into_response()
}
// endregion: --- About page

// region:	  --- Page Not Found
pub fn not_found_page() -> Response {
    NotFoundTemplate {
        title: "Not Found".to_string(),
    }
    .into_response()
}
// endregion: --- Page Not Found

// region:	  --- Test response
#[derive(Template)]
#[template(path = "test_response.html")]
struct TestResponseTemplate {
    response: String,
}

pub fn test_response(response: &str) -> Response {
    TestResponseTemplate {
        response: response.to_string(),
    }
    .into_response()
}
// endregion: --- Test response
// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
}
// endregion: --- Tests
