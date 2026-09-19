mod auth;
mod deposit;
mod health_check;
mod webhooks;

use crate::*;

use axum::extract::*;

const BODY_MAX_SIZE: usize = 1024 * 1024 * 1; // 1 MB

pub fn get_routes(app: &app::AppState) -> axum::Router<app::AppState> {
	let cors = tower_http::cors::CorsLayer::new()
		.allow_headers(tower_http::cors::Any)
		.allow_origin(tower_http::cors::Any)
		.allow_methods(tower_http::cors::Any);

	let body_limit = DefaultBodyLimit::max(BODY_MAX_SIZE);

	axum::Router::new()
		.nest("/deposit", deposit::router(app))
		.nest("/auth", auth::router())
		.nest("/webhook", webhooks::router(app))
		.merge(health_check::router())
		.layer(cors)
		.layer(body_limit)
		.with_state(app.clone())
}
