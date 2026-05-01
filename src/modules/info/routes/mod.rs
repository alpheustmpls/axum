use jder_axum::response::{Response, json::CreateJsonResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    common::structs::response::JsonSuccessResponse,
    modules::info::{schema::Info, services::service_info},
};

/// Info
///
/// The info page of the API.
///
/// Contains information about the API.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/info",
    operation_id = "get_info",
    responses(
        (
            status = 200,
            body = JsonSuccessResponse<Info>,
        ),
    )
)]
async fn route_info() -> Response {
    match service_info().await {
        | Ok(data) => CreateJsonResponse::success().data(data).create(),
        | Err(err) => err.into_failure_response(),
    }
}

pub fn router_info() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(route_info))
}
