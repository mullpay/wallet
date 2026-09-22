use crate::*;

use axum::extract::*;
use axum::routing::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebHookEvent {
	#[serde(rename = "qrId")]
	pub deposit_id: uuid::Uuid,
	#[serde(rename = "payerEUID")]
	pub payer_euid: String,
	pub status: depix_api::DepositStatus,
	pub pix_key: String,
}

pub fn router(app: &app::AppState) -> axum::Router<app::AppState> {
	axum::Router::new()
		.route("/deposit", post(deposit_route))
		.layer(axum::middleware::from_fn_with_state(app.clone(), middlewares::webhook_middleware))
}

async fn deposit_route(app: State<app::AppState>, payload: Json<WebHookEvent>) -> error::Result<()> {
	let status = payload.status.into();
	let deposit_id = payload.deposit_id;
	let eulen_id = &payload.payer_euid;

	let deposit = db::deposit::update_status_by_deposit_id(&app.pool, deposit_id, status).await?;

	db::account::update_eulen_id(&app.pool, deposit.account_id, Some(eulen_id)).await?;

	Ok(())
}
