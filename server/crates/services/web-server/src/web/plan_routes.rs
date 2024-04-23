use crate::web::routes_static::{not_found, not_found_handler};
use crate::web::{Error, Result};
use ::time::Date;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderValue, StatusCode, Uri};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use lib_core::ctx::Ctx;
use lib_core::model::plan::{self, Plan, PlanBmc, PlanForCreate};
use lib_core::model::ModelManager;
use lib_html::plan_template::plan_page;
use lib_html::{about_page, home_page, test_response};
use lib_utils::time;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
    Router::new().nest(
        "/plan",
        Router::new()
            .route("/", post(create_plan_handler))
            .route(
                "/:plan_slug",
                get(plan_page_handler).post(toggle_date_handler),
            )
            .with_state(mm.clone()),
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
    let plan = PlanBmc::get_plan_by_url(&Ctx::root_ctx(), &mm, &page_slug)
        .await
        .map_err(Error::Model)?;

    if let Some(plan) = plan {
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
    // -- Check if name length is not too long
    if new_plan.new_plan.len() > 128 {
        let mut too_long_response = Response::new(Body::empty());
        *too_long_response.status_mut() = StatusCode::BAD_REQUEST;

        return Ok(too_long_response);
    }

    // -- Create new url_id
    let url_id = lib_utils::url_id::new_url_id();

    // -- Create the plan with the BMC
    let root_ctx = Ctx::root_ctx();
    let created_plan = PlanBmc::create(
        &root_ctx,
        &mm,
        PlanForCreate {
            name: new_plan.new_plan,
            url_id: url_id.clone(),
            description: None,
        },
    )
    .await?;

    // -- Add the HX-Push-Url header to switch out the URL.
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
struct ToggleDate {
    #[serde(with = "date_format")]
    date: Date,
}

async fn toggle_date_handler(
    Path(page_slug): Path<String>,
    State(mm): State<ModelManager>,
    Form(toggle_date): Form<ToggleDate>,
) -> Result<Response> {
    debug!(
        "{:<12} - toggle_date_handler - {} - {}",
        "HANDLER", page_slug, toggle_date.date
    );

    Ok(test_response("nothing"))
}
// endregion: --- Date operations
