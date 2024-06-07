use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::post;
use axum::{Form, Router};
use lib_core::model::plan::PlanBmc;
use lib_core::model::user::{UserBmc, UserForCreate};
use lib_core::model::ModelManager;
use lib_html::plan_template::user_created_div;
use serde::Deserialize;
use tracing::debug;

use crate::web::Result;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/user/:plan_slug", post(create_user_handler))
        .with_state(mm)
}

// region:	  --- User creation
#[derive(Deserialize)]
struct NewUser {
    username: String,
}

async fn create_user_handler(
    State(mm): State<ModelManager>,
    Path(page_slug): Path<String>,
    Form(new_user): Form<NewUser>,
) -> Result<Response> {
    debug!(
        "{:<12} - create_user_handler - {} - {}",
        "HANDLER", page_slug, new_user.username
    );

    // -- Setup ctx
    // -- Validation
    // Check if name length is not too long (saves a trip to the DB)
    if new_user.username.len() > 128 {
        let mut too_long_response = Response::new(Body::empty());
        *too_long_response.status_mut() = StatusCode::BAD_REQUEST;

        return Ok(too_long_response);
    }
    // Check if plan exists
    let plan = PlanBmc::get_plan_by_url(&mm, &page_slug).await?;

    // Check if the name already exists for the plan
    // Not necessary since the DB does this check.

    // -- Creation
    let user = UserBmc::create_return(
        &mm,
        UserForCreate {
            name: new_user.username,
            plan_id: plan.id,
        },
    )
    .await?;

    Ok(user_created_div(user.name, user.web_id))
}

// endregion: --- User creation
