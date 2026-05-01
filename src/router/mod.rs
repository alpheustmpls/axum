use std::time::Duration;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    handler::HandlerWithoutStateExt,
    http::{HeaderValue, StatusCode, header, request::Parts},
    routing::get,
};
use jder_axum::{
    layers::{RequestBodyLimit, RequestTimeLimit},
    response::{
        Response,
        json::{CreateJsonResponse, JsonResponseError},
    },
};
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, Any, CorsLayer},
    services::ServeDir,
};
use utoipa::openapi::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable as _};

use crate::{
    common::structs::response::JsonSuccessResponse,
    constants::path::get_app_root, modules::info::routes::router_info,
};

/// Index
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/",
    operation_id = "get_index",
    responses(
        (
            status = 200,
            body = JsonSuccessResponse,
        ),
    )
)]
async fn route_index() -> Response {
    CreateJsonResponse::dataless().create()
}

/// Build OpenAPI router + spec
fn build_openapi() -> (Router, OpenApi) {
    OpenApiRouter::new()
        .routes(routes!(route_index))
        .merge(router_info())
        .split_for_parts()
}

/// Serve OpenAPI JSON
async fn route_openapi() -> Response {
    let (_, api) = build_openapi();

    CreateJsonResponse::success::<String>()
        .data(api.to_json().expect("failed to serialize OpenAPI spec"))
        .create()
}

/// Not Found
async fn route_not_found() -> Response {
    CreateJsonResponse::failure()
        .status(StatusCode::NOT_FOUND)
        .add_error(
            JsonResponseError::new()
                .code("not_found")
                .message("Content not found"),
        )
        .create()
}

/// CORS configuration
fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_headers(Any)
        .allow_headers([header::SET_COOKIE, header::CONTENT_TYPE])
        .allow_origin(AllowOrigin::predicate(|_: &HeaderValue, _: &Parts| true))
}

/// Create router for the app
pub async fn create_router() -> Router {
    let (router, api) = build_openapi();

    Router::new()
        .merge(router)
        .route("/openapi.json", get(route_openapi))
        .merge(Scalar::with_url("/openapi", api))
        .fallback_service(
            ServeDir::new(get_app_root().await.join("public"))
                .not_found_service(route_not_found.into_service()),
        )
        .layer(CompressionLayer::new())
        .layer(cors_layer())
        .layer(RequestTimeLimit::max(Duration::from_secs(10)))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimit::max(100 * 1024 * 1024))
}
