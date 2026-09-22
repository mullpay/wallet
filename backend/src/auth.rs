use crate::*;

use axum::*;

pub fn get_token(headers: &http::HeaderMap) -> Option<&str> {
	if let Some(value) = headers.get("Authorization") {
		if let Ok(value) = value.to_str() {
			return value.trim().get(7..);
		}
	};

	None
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChallengePayload {
	pub public_key: crypto::PublicKey,
	pub nonce: [u8; 32],
	pub exp: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthenticatedPayload {
	pub public_key: crypto::PublicKey,
	pub account_id: uuid::Uuid,
	pub exp: u64,
}
