use ::time::Date;
use askama::Template;
use axum::response::{IntoResponse, Response};
use lib_core::model::plan::Plan;
use lib_utils::time::{self, current_date};

use crate::calendar_utils::get_calender_month_dates;

#[derive(Template)]
#[template(path = "plan.html")]
struct PlanTemplate {
    calendar: Calendar,
    title: String,
    plan_name: String,
    description: String,
}

// region:	  --- Calendar struct
struct Calendar {
    current_date: Date,
    dates: Vec<Vec<Date>>,
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
        // -- Calendar data for month of the current date
        let dates = get_calender_month_dates(current_date().month(), current_date().year())
            .chunks(7)
            .map(|week| week.into())
            .collect();

        let calendar_data = Calendar {
            current_date: current_date(),
            dates,
            selected_dates: vec![],
        };

        PlanTemplate {
            title: plan.name.clone(),
            plan_name: plan.name,
            calendar: calendar_data,
            description: plan.description.unwrap_or("".to_string()),
        }
    }
}
