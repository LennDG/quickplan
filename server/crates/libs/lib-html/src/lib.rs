use std::collections::HashMap;

use askama::Template;
use axum::response::{IntoResponse, Response};
use time::Date;

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

#[derive(Template)]
#[template(path = "plan.html")]
struct PlanTemplate {
    title: String,
    plan_name: String,
    calendar: CalendarTemplate,
}
// endregion: --- Page structs

// region:	  --- Calendar struct
#[derive(Template)]
#[template(path = "test_calendar.html")]
struct CalendarTemplate {
    current_date: Date,
    selected_dates: Vec<(String, Date)>, // Username + Date
}
// endregion: --- Calendar struct

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

// region:	  --- Page Not Found
pub fn not_found_page() -> Response {
    NotFoundTemplate {
        title: "Not Found".to_string(),
    }
    .into_response()
}
// endregion: --- Page Not Found

// region:	  --- Plan page
pub fn plan_page(plan_name: String, current_date: Date) -> Response {
    PlanTemplate {
        title: plan_name.clone(),
        plan_name,
        calendar: CalendarTemplate {
            current_date,
            selected_dates: vec![],
        },
    }
    .into_response()
}
// endregion: --- Plan page

// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;
}
// endregion: --- Tests
