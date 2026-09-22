use crate::*;

use axum::extract::*;
use axum::routing::*;

const EULEN_FEE_IN_CENTS: u32 = 99;
const INTERNAL_FEE_PERCENT: Decimal = dec!(2.25);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetLimitResponse {
	pub max_deposit_amount_in_cents: u32,
	pub min_deposit_amount_in_cents: u32,
	pub max_payout_amount_in_cents: u32,
	pub min_payout_amount_in_cents: u32,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DepositRequest {
	#[serde(flatten)]
	pub amount: DepositAmount,
	pub address: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum DepositAmount {
	DepositAmountInCents { deposit_amount_in_cents: u32 },
	PayoutAmountInCents { payout_amount_in_cents: u32 },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DepositResponse {
	pub br_code: String,
	pub deposit_amount_in_cents: u32,
	pub payout_amount_in_cents: u32,
	pub fee_amount_in_cents: u32,
}

pub fn router(app: &app::AppState) -> axum::Router<app::AppState> {
	axum::Router::new()
		.route("/limit", get(get_limit_route))
		.route("/", post(deposit_route))
		.layer(axum::middleware::from_fn_with_state(app.clone(), middlewares::auth_middleware))
}

async fn deposit_route(app: State<app::AppState>, auth: Extension<auth::AuthenticatedPayload>, payload: Json<DepositRequest>) -> error::Result<Json<DepositResponse>> {
	let account = db::account::get_account(&app.pool, &auth.account_id).await?;

	let (deposit_amount_in_cents, payout_amount_in_cents, fee_amount_in_cents) = match &payload.amount {
		DepositAmount::DepositAmountInCents { deposit_amount_in_cents: value } => {
			let multiplier = INTERNAL_FEE_PERCENT / Decimal::ONE_HUNDRED;
			let fee: u32 = (Decimal::from(*value) * multiplier).ceil().try_into().unwrap();
			let total_fee = fee + EULEN_FEE_IN_CENTS;

			(*value, value.saturating_sub(total_fee), total_fee)
		}
		DepositAmount::PayoutAmountInCents { payout_amount_in_cents: value } => {
			let multiplier = INTERNAL_FEE_PERCENT / Decimal::ONE_HUNDRED;
			let fee: u32 = (Decimal::from(*value) * multiplier).ceil().try_into().unwrap();
			let total_fee = fee + EULEN_FEE_IN_CENTS;

			(value.saturating_add(total_fee), *value, total_fee)
		}
	};

	let (min_deposit_amount_in_cents, max_deposit_amount_in_cents, _, _) = calc_limit(&app, account.eulen_id.as_deref()).await?;

	if !(min_deposit_amount_in_cents..=max_deposit_amount_in_cents).contains(&deposit_amount_in_cents) {
		return Err(error::Error::InsufficientLimit);
	}

	let split = depix_api::SplitFee {
		percent: INTERNAL_FEE_PERCENT,
		address: app.env.spark_address.clone(),
	};

	let deposit = depix_api::deposit_pix(&app.env, deposit_amount_in_cents as u32, Some(&payload.address), account.eulen_id.as_deref(), Some(split)).await?;
	let status = depix_api::get_deposit_status(&app.env, deposit.id).await?;

	db::deposit::insert_deposit(
		&app.pool,
		auth.account_id,
		deposit.id,
		status.status.into(),
		deposit_amount_in_cents,
		payout_amount_in_cents,
		fee_amount_in_cents,
	)
	.await?;

	Ok(Json(DepositResponse {
		br_code: deposit.qr_copy_paste,
		deposit_amount_in_cents,
		payout_amount_in_cents,
		fee_amount_in_cents,
	}))
}

async fn get_limit_route(app: State<app::AppState>, auth: Extension<auth::AuthenticatedPayload>) -> error::Result<Json<GetLimitResponse>> {
	let account = db::account::get_account(&app.pool, &auth.account_id).await?;
	let (min_deposit_amount_in_cents, max_deposit_amount_in_cents, min_payout_amount_in_cents, max_payout_amount_in_cents) = calc_limit(&app, account.eulen_id.as_deref()).await?;

	Ok(Json(GetLimitResponse {
		min_deposit_amount_in_cents,
		max_deposit_amount_in_cents,
		min_payout_amount_in_cents,
		max_payout_amount_in_cents,
	}))
}

async fn calc_limit(app: &app::AppState, eulen_id: Option<&str>) -> error::Result<(u32, u32, u32, u32)> {
	let (min_deposit_amount_in_cents, max_deposit_amount_in_cents) = match eulen_id {
		Some(id) => {
			let info = depix_api::get_user_info(&app.env, id).await?;

			let max_deposit_amount_in_cents = info.max_daily_in_cents.saturating_sub(info.daily_volume_in_cents);
			let min_deposit_amount_in_cents = 150u32;

			(min_deposit_amount_in_cents, max_deposit_amount_in_cents)
		}
		None => {
			let max_deposit_amount_in_cents = 1000u32;
			let min_deposit_amount_in_cents = 150u32;

			(min_deposit_amount_in_cents, max_deposit_amount_in_cents)
		}
	};

	let multiplier = Decimal::ONE + INTERNAL_FEE_PERCENT / Decimal::ONE_HUNDRED;

	let max_payout_amount_in_cents: u32 = (Decimal::from(max_deposit_amount_in_cents.saturating_sub(EULEN_FEE_IN_CENTS)) / multiplier).floor().try_into().unwrap();
	let min_payout_amount_in_cents: u32 = (Decimal::from(min_deposit_amount_in_cents.saturating_sub(EULEN_FEE_IN_CENTS)) / multiplier).floor().try_into().unwrap();

	Ok((min_deposit_amount_in_cents, max_deposit_amount_in_cents, min_payout_amount_in_cents, max_payout_amount_in_cents))
}
