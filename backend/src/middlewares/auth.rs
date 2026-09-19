use crate::*;

use axum::extract::*;
use axum::*;

pub fn get_token(headers: &http::HeaderMap) -> Option<&str> {
	if let Some(value) = headers.get("Authorization") {
		if let Ok(value) = value.to_str() {
			return value.trim().split_once(' ').map(|(_, value)| value.trim());
		}
	};

	None
}

pub async fn auth_middleware(app: State<app::AppState>, mut req: Request, next: middleware::Next) -> error::Result<response::Response> {
	let Some(token) = get_token(req.headers()) else {
		return Err(error::Error::Unauthorized);
	};

	let Ok(token_data) = crypto::decode_jwt::<auth::AuthenticatedPayload>(app.env.jwt_secret.as_bytes(), token) else {
		return Err(error::Error::Unauthorized);
	};

	req.extensions_mut().insert(token_data);

	Ok(next.run(req).await)
}

pub async fn webhook_middleware(app: State<app::AppState>, req: Request, next: middleware::Next) -> error::Result<response::Response> {
	let Some(secret) = get_token(req.headers()) else {
		return Err(error::Error::Unauthorized);
	};

	if secret != app.env.webhook_secret {
		return Err(error::Error::Unauthorized);
	}

	Ok(next.run(req).await)
}
