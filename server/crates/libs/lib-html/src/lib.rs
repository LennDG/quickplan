use axum::response::{IntoResponse, Response};
use maud::{html, Markup, Render, DOCTYPE};

// region:	  --- Modules
pub mod error;

pub use self::error::{Error, Result};
// endregion: --- Modules

// region:	  --- Page
pub fn page(title: &str, content: Markup) -> Markup {
    /// A basic header with a dynamic `page_title`.
    pub(crate) fn head(page_title: &str) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en";
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" type="text/css" href="/style.css";
                base href="http://localhost:8080/";
                script src="https://unpkg.com/htmx.org@1.9.10" {};
                script type="module" src="script-twind.js" {};
                title { "Quickplan - " (page_title) }

            }
        }
    }

    pub(crate) fn header() -> Markup {
        html! {
            header ."container py-5 flex flex-row place-content-center gap-6 items-center" {}
        }
    }

    /// A static footer.
    pub(crate) fn footer() -> Markup {
        html! {}
    }

    html! {
        (head(title))
        body ."container relative mx-auto !block" style="display: none" autofocus {
            (header())
            main ."container" {
                (content)
            }
            (footer())
        }
    }
}

// endregion: --- Page

// region:	  --- Landing page
pub fn landing_page() -> Response {
    let html = html!(
        h1 {"Hello World!"}
    );

    page("Landing Page", html).into_response()
}
// endregion: --- Landing page

// region:	  --- Plan page
pub fn plan_page(plan_name: String) -> Response {
    let html = html!(
        h1 {"Hello " (plan_name) "!"}
    );

    page("Plan Page", html).into_response()
}
// endregion: --- Plan page

// region:	  --- Page Not Found
pub fn page_not_found() -> Response {
    let html = html!(
        h1 {"PAGE NOT FOUND"}
        a href="http://localhost:8080/" { "Back To Start" }
    );

    page("Not Found", html).into_response()
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
