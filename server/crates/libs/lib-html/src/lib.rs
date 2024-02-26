use askama::Template;
use axum::response::{IntoResponse, Response};
use maud::{html, Markup, Render, DOCTYPE};

// region:	  --- Modules
pub mod error;

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
        title: "Quickplan!".to_string(),
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

// region:	  --- Plan page
pub fn plan_page(plan_name: String) -> Response {
    todo!()
}
// endregion: --- Plan page

// region:	  --- Page Not Found
pub fn page_not_found() -> Response {
    NotFoundTemplate {
        title: "Not Found".to_string(),
    }
    .into_response()
}
// endregion: --- Page Not Found

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_html_gen() -> Result<()> {
        let name = "Lyra";
        let markup = html! {
            p {  "<script>alert(\"XSS\")</script>"}
        };
        println!("{}", markup.into_string());

        Ok(())
    }
}
// endregion: --- Tests
