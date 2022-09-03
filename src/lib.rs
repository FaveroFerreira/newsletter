use axum::routing::get;
use axum::Router;

mod routes;

pub fn configure_router() -> Router {
    Router::new().route(
        routes::health_check::ENDPOINT,
        get(routes::health_check::handler),
    )
}
