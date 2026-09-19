use crate::*;

const API_URL: &str = "https://depix.eulen.app/api";

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DepixResponse<T: serde::Serialize, E: serde::Serialize = ErrorResponse> {
	response: DepixResult<T, E>,
	r#async: bool,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", untagged)]
enum DepixResult<T: serde::Serialize, E: serde::Serialize> {
	Ok(T),
	Err(E),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePixResponse {
	pub id: uuid::Uuid,
	pub qr_copy_paste: String,
	pub qr_image_url: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
	pub error_message: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositStatusResponse {
	pub status: DepositStatus,
	pub value_in_cents: u32,
	#[serde(with = "time::serde::rfc3339")]
	pub expiration: time::OffsetDateTime,
	#[serde(rename = "payerEUID")]
	pub payer_euid: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserInfoResponse {
	pub daily_volume_in_cents: u32,
	pub max_daily_in_cents: u32,
	pub daily_limit_reset_time: String,
	pub is_blocked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DepositStatus {
	Pending,
	UnderReview,
	Approved,
	DepixSent,
	Delayed,
	Refunded,
	Canceled,
	Expired,
	Error,
	Unknown,
	WillRefund,
}

pub struct SplitFee {
	pub percent: Decimal,
	pub address: String,
}

pub async fn deposit_pix(env: &env::Env, amount_in_cents: u32, depix_address: Option<&str>, euid: Option<&str>, split: Option<SplitFee>) -> error::Result<CreatePixResponse> {
	let json = match split {
		Some(split) => serde_json::json!({
			"depixAddress": depix_address,
			"amountInCents": amount_in_cents,
			"euid": euid,
			"splitFee": format!("{}%", split.percent),
			"depixSplitAddress": split.address
		}),
		None => serde_json::json!({
			"depixAddress": depix_address,
			"amountInCents": amount_in_cents,
			"euid": euid
		}),
	};

	Ok(reqwest::Client::new()
		.post(format!("{API_URL}/deposit"))
		.bearer_auth(&env.depix_token)
		.json(&json)
		.send()
		.await?
		.json::<DepixResponse<CreatePixResponse>>()
		.await?
		.to_result()?)
}

pub async fn get_deposit_status(env: &env::Env, pix_id: uuid::Uuid) -> error::Result<GetDepositStatusResponse> {
	Ok(reqwest::Client::new()
		.get(format!("{API_URL}/deposit-status?id={pix_id}"))
		.bearer_auth(&env.depix_token)
		.send()
		.await?
		.json::<DepixResponse<_>>()
		.await?
		.to_result()?)
}

pub async fn get_user_info(env: &env::Env, euid: &str) -> error::Result<GetUserInfoResponse> {
	Ok(reqwest::Client::new()
		.get(format!("{API_URL}/user-info?euid={euid}"))
		.bearer_auth(&env.depix_token)
		.send()
		.await?
		.json::<DepixResponse<_>>()
		.await?
		.to_result()?)
}

impl<T: serde::Serialize, E: serde::Serialize> DepixResponse<T, E> {
	#[inline(always)]
	pub fn to_result(self) -> Result<T, E> {
		match self.response {
			DepixResult::Ok(t) => Result::Ok(t),
			DepixResult::Err(e) => Result::Err(e),
		}
	}
}
