use ::time::{Date, Month};
use askama::Template;
use axum::response::{IntoResponse, Response};
use lib_core::model::plan::Plan;
use lib_utils::time::{self, current_date};

use crate::calendar_utils::get_calender_month_dates;

// region:	  --- Plan template
#[derive(Template)]
#[template(path = "plan.html")]
struct PlanTemplate {
    calendar: Calendar,
    title: String,
    plan_name: String,
    description: String,
}

impl From<Plan> for PlanTemplate {
    fn from(plan: Plan) -> Self {
        PlanTemplate {
            title: plan.name.clone(),
            plan_name: plan.name,
            calendar: Calendar::new(current_date().month(), current_date().year()),
            description: plan.description.unwrap_or("".to_string()),
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
    month: Month,
    year: i32,
}

impl Calendar {
    fn new(month: Month, year: i32) -> Self {
        let weeks = get_calender_month_dates(month, year)
            .chunks(7)
            .map(|week| week.into())
            .collect();

        Calendar {
            current_date: current_date(),
            month,
            year,
            weeks,
        }
    }
}
// endregion: --- Calendar struct

// region:	  --- Plan page
pub fn plan_page(plan: Plan) -> Response {
    PlanTemplate::from(plan).into_response()
}

pub fn calendar_div(month: Month, year: i32) -> Response {
    CalendarTemplate {
        calendar: Calendar::new(month, year),
    }
    .into_response()
}
// endregion: --- Plan page
