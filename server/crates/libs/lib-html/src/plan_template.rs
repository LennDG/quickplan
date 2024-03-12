use askama::Template;
use axum::response::{IntoResponse, Response};
use lib_core::model::plan::Plan;
use time::Date;

#[derive(Template)]
#[template(path = "plan.html")]
struct PlanTemplate {
    title: String,
    plan_name: String,
    calendar: CalendarTemplate,
    description: String,
}

// region:	  --- Calendar struct
#[derive(Template)]
#[template(path = "test_calendar.html")]
struct CalendarTemplate {
    current_date: Date,
    selected_dates: Vec<(String, Date)>, // Username + Date
}
// endregion: --- Calendar struct

// region:	  --- Plan page
pub fn plan_page(plan: Plan) -> Response {
    PlanTemplate::from(plan).into_response()
}
// endregion: --- Plan page

impl From<Plan> for PlanTemplate {
    fn from(plan: Plan) -> Self {
        PlanTemplate {
            title: plan.name.clone(),
            plan_name: plan.name,
            calendar: CalendarTemplate {
                current_date: lib_utils::time::current_date(),
                selected_dates: vec![],
            },
            description: plan.description.unwrap_or("".to_string()),
        }
    }
}
