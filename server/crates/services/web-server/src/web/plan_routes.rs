use crate::web::routes_static::not_found_handler;
use crate::web::{Error, Result};
use ::time::{Date, Month};
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderValue, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Form, Router};
use lib_core::model::fields::WebId;
use lib_core::model::plan::{PlanBmc, PlanForCreate};
use lib_core::model::user::UserBmc;
use lib_core::model::user_date::{UserDateBmc, UserDateForCreate};
use lib_core::model::ModelManager;
use lib_html::plan_template::{calendar_div, plan_page};
use lib_html::test_response;
use serde::Deserialize;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().nest(
        "/plan",
        Router::new()
            .route("/", post(create_plan_handler))
            .route("/calendar", get(calendar_month_selection_handler))
            .nest(
                "/:plan_slug",
                Router::new()
                    .route("/", get(plan_page_handler))
                    .route("/", post(toggle_date_handler)),
            )
            .with_state(mm),
    )
}

// region:	  --- Plan Page
async fn plan_page_handler(
    State(mm): State<ModelManager>,
    Path(page_slug): Path<String>,
    uri: Uri,
) -> Result<Response> {
    debug!("{:<12} - plan_page_handler - {page_slug}", "HANDLER");
    // -- Check if the page exists
    let plan = PlanBmc::get_plan_by_url(&mm, &page_slug)
        .await
        .map_err(Error::Model);

    if let Ok(plan) = plan {
        Ok(plan_page(plan))
    } else {
        Ok(not_found_handler(uri).await)
    }
}
// endregion: --- Plan Page

// region:	  --- Plan creation
#[derive(Deserialize)]
struct NewPlan {
    new_plan: String,
}

async fn create_plan_handler(
    State(mm): State<ModelManager>,
    Form(new_plan): Form<NewPlan>,
) -> Result<Response> {
    debug!(
        "{:<12} - create_plan_handler - {}",
        "HANDLER", new_plan.new_plan
    );
    // -- Check if name length is not too long (saves a DB trip)
    if new_plan.new_plan.len() > 128 {
        let mut too_long_response = Response::new(Body::empty());
        *too_long_response.status_mut() = StatusCode::BAD_REQUEST;

        return Ok(too_long_response);
    }

    // -- Create new url_id
    let url_id = lib_utils::url_id::new_url_id();

    // -- Create the plan with the BMC
    PlanBmc::create(
        &mm,
        PlanForCreate {
            name: new_plan.new_plan,
            url_id: url_id.clone(),
            description: None,
        },
    )
    .await?;

    // -- Add the HX-Redirect header to redirect to different page.
    // TODO: Is there a more "HTML" friendly way to do this?
    let mut plan_response = Response::new(Body::empty());
    plan_response.headers_mut().append(
        "HX-Redirect",
        HeaderValue::from_str(("plan/".to_owned() + &url_id).as_str()).unwrap(),
    );
    *plan_response.status_mut() = StatusCode::CREATED;

    Ok(plan_response)
}
// endregion: --- Plan creation

// region:	  --- Date operations
::time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Deserialize)]
struct ToggleUserDate {
    #[serde(with = "date_format")]
    date: Date,
    user_id: WebId,
}

async fn toggle_date_handler(
    Path(page_slug): Path<String>,
    State(mm): State<ModelManager>,
    Form(toggle_user_date): Form<ToggleUserDate>,
) -> Result<Response> {
    debug!(
        "{:<12} - toggle_date_handler - {} - {} - {} ",
        "HANDLER", page_slug, toggle_user_date.date, toggle_user_date.user_id
    );

    // -- Get the internal user id
    let user = UserBmc::get_user_with_web_id(&mm, toggle_user_date.user_id).await?;
    let date_c = UserDateForCreate {
        date: toggle_user_date.date.into(),
        user_id: user.id,
    };

    // -- Toggle the date
    let user_date_id = UserDateBmc::get_date(&mm, date_c.clone()).await?;
    if let Some(id) = user_date_id {
        UserDateBmc::delete(&mm, id).await?;
    } else {
        UserDateBmc::create(&mm, date_c).await?;
    }

    Ok(test_response("nothing"))
}
// endregion: --- Date operations

// region:	  --- Calendar operations
#[derive(Deserialize)]
struct PlanCalendar {
    month: Month,
    year: i32,
    plan_id: String,
}

async fn calendar_month_selection_handler(
    State(mm): State<ModelManager>,
    Query(plan_calendar): Query<PlanCalendar>,
) -> Result<Response> {
    debug!(
        "{:<12} - calendar_month_selection_handler - {} - {} - {}",
        "HANDLER", plan_calendar.plan_id, plan_calendar.month, plan_calendar.year
    );

    Ok(calendar_div(
        plan_calendar.month,
        plan_calendar.year,
        vec![],
        vec![],
    ))
}
// endregion: --- Calendar operations
