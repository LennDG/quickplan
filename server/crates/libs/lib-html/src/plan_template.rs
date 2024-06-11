use ::time::{Date, Month};
use askama::Template;
use axum::response::{IntoResponse, Response};
use lib_core::model::fields::WebId;
use lib_core::model::plan::Plan;
use lib_utils::time::current_date;

use crate::calendar_utils::{calender_month_dates, next_calendar_month, previous_calendar_month};

// region:	  --- Plan template
#[derive(Template)]
#[template(path = "plan.html")]
struct PlanTemplate {
    calendar: Calendar,
    title: String,
    plan_name: String,
    description: String,
    plan_id: String,
    users: Vec<String>,
}

impl From<Plan> for PlanTemplate {
    fn from(plan: Plan) -> Self {
        PlanTemplate {
            title: plan.name.clone(),
            plan_name: plan.name,
            calendar: Calendar::new(
                current_date().month(),
                current_date().year(),
                vec![],
                vec![],
            ),
            description: plan.description.unwrap_or_default(),
            plan_id: plan.url_id,
            users: vec![],
        }
    }
}
// endregion: --- Plan template

// region:	  --- Calendar struct
#[derive(Template)]
#[template(path = "calendar.html")]
struct CalendarTemplate {
    calendar: Calendar,
}

struct Calendar {
    current_date: Date,
    weeks: Vec<Vec<Date>>,
    plan_selected_dates: Vec<Date>,
    user_selected_dates: Vec<Date>,
    month: Month,
    year: i32,
    next_month: String,
    prev_month: String,
    next_year: i32,
    prev_year: i32,
}

impl Calendar {
    fn new(
        month: Month,
        year: i32,
        plan_selected_dates: Vec<Date>,
        user_selected_dates: Vec<Date>,
    ) -> Self {
        let weeks = calender_month_dates(month, year)
            .chunks(7)
            .map(|week| week.into())
            .collect();

        let (next_month, next_year) = next_calendar_month(&month, year);
        let (prev_month, prev_year) = previous_calendar_month(&month, year);

        Calendar {
            current_date: current_date(),
            weeks,
            // Can unwrap because this serialization cannot fail
            next_month: serde_json::to_string(&next_month).unwrap(),
            prev_month: serde_json::to_string(&prev_month).unwrap(),
            next_year,
            prev_year,
            month,
            year,
            user_selected_dates,
            plan_selected_dates,
        }
    }
}

// endregion: --- Calendar struct

// region:	  --- User structs
#[derive(Template)]
#[template(path = "user_created_response.html")]
struct UserCreatedResponseTemplate {
    username: String,
    user_id: WebId,
}
// endregion: --- User structs

// region:	  --- Plan page
pub fn plan_page(plan: Plan) -> Response {
    PlanTemplate::from(plan).into_response()
}

pub fn calendar_div(
    month: Month,
    year: i32,
    plan_selected_dates: Vec<Date>,
    user_selected_dates: Vec<Date>,
) -> Response {
    CalendarTemplate {
        calendar: Calendar::new(month, year, plan_selected_dates, user_selected_dates),
    }
    .into_response()
}

pub fn user_created_div(username: String, user_id: WebId) -> Response {
    UserCreatedResponseTemplate { username, user_id }.into_response()
}
// endregion: --- Plan page
