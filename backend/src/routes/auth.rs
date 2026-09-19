use crate::*;

use axum::extract::*;
use axum::routing::*;

use std::time;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct RequestChallengeRequest {
	pub public_key: crypto::PublicKey,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct RequestChallengeResponse {
	pub token: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct ConfirmChallengeRequest {
	pub token: String,
	pub signature: crypto::Signature,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct ConfirmChallengeResponse {
	pub token: String,
	pub payload: auth::AuthenticatedPayload,
}

pub fn router() -> axum::Router<app::AppState> {
	axum::Router::new()
		.route("/request-challenge", post(request_challenge_route))
		.route("/confirm-challenge", post(confirm_challenge_route))
}

async fn request_challenge_route(app: State<app::AppState>, payload: Json<RequestChallengeRequest>) -> error::Result<Json<RequestChallengeResponse>> {
	let nonce = crypto::rand32();
	let public_key = payload.public_key;

	let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)?;
	let exp = now.as_secs() + app.env.jwt_challenge_expiration_seconds;

	let challenge_payload = auth::ChallengePayload { public_key, nonce, exp };
	let token = crypto::encode_jwt(app.env.jwt_challenge_secret.as_bytes(), &challenge_payload)?;

	Ok(Json(RequestChallengeResponse { token }))
}

async fn confirm_challenge_route(app: State<app::AppState>, payload: Json<ConfirmChallengeRequest>) -> error::Result<Json<ConfirmChallengeResponse>> {
	let token = &payload.token;
	let signature = &payload.signature;
	let token_hash = crypto::sha256(token.as_bytes());
	let challenge_payload = crypto::decode_jwt::<auth::ChallengePayload>(app.env.jwt_challenge_secret.as_bytes(), token)?;
	let public_key = challenge_payload.public_key;

	let valid = public_key.verify(*token_hash, &signature);

	if valid.is_err() {
		return Err(error::Error::Unauthorized);
	}

	let account = db::account::get_or_insert_account(&app.pool, &public_key).await?;
	let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH)?;
	let exp = now.as_secs() + app.env.jwt_expiration_seconds;
	let account_id = account.id;

	let payload = auth::AuthenticatedPayload { public_key, account_id, exp };
	let token = crypto::encode_jwt(app.env.jwt_secret.as_bytes(), &payload)?;

	Ok(Json(ConfirmChallengeResponse { token, payload }))
}
