use crate::*;

use axum::routing::*;

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new().route("/", get(root))
}

async fn root() -> &'static str {
	"OK"
}
