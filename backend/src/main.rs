pub mod app;
pub mod auth;
pub mod crypto;
pub mod db;
pub mod depix_api;
pub mod env;
pub mod error;
pub mod middlewares;
pub mod routes;

use rust_decimal::Decimal;
use rust_decimal::dec;

#[tokio::main]
async fn main() -> error::Result<()> {
	simple_logger::init_with_level(log::Level::Info).unwrap();

	let app_state = app::AppState::init().await?;

	let listener = tokio::net::TcpListener::bind(&format!("{}:{}", app_state.env.server_addr, app_state.env.server_port)).await?;
	let router = routes::get_routes(&app_state).with_state(app_state);

	Ok(axum::serve(listener, router).await?)
}
