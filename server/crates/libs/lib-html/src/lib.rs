use axum::response::{IntoResponse, Response};
use maud::{html, Markup, Render};

// region:	  --- Modules
pub mod error;

pub use self::error::{Error, Result};
// endregion: --- Modules

// region:	  --- CSS
/// Links to a CSS stylesheet at the given path.
struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}
// endregion: --- CSS

// region:	  --- Landing page
pub fn landing_page() -> Result<Response> {
    let html = html!(
        h1 {"Hello World!"}
    );

    Ok(html.into_response())
}
// endregion: --- Landing page

fn test() {
    let name = "Lyra";
    let markup = html! {
        p { "Hi, " (name) "!" }
    };
    println!("{}", markup.into_string());
}

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
