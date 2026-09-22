use crate::*;

#[derive(Debug, Clone)]
pub struct Deposit {
	pub id: uuid::Uuid,
	pub account_id: uuid::Uuid,
	pub deposit_id: uuid::Uuid,
	pub status: DepositStatus,
	pub deposit_amount_in_cents: u32,
	pub payout_amount_in_cents: u32,
	pub fee_amount_in_cents: u32,
	pub created_at: time::OffsetDateTime,
	pub updated_at: time::OffsetDateTime,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
pub enum DepositStatus {
	Pending = 0,
	UnderReview = 1,
	Approved = 2,
	DepixSent = 3,
	Delayed = 4,
	Refunded = 5,
	Canceled = 6,
	Expired = 7,
	Error = 8,
	Unknown = 9,
	WillRefund = 10,
}

pub async fn insert_deposit(
	pool: impl sqlx::SqliteExecutor<'_>,
	account_id: uuid::Uuid,
	deposit_id: uuid::Uuid,
	status: DepositStatus,
	deposit_amount_in_cents: u32,
	payout_amount_in_cents: u32,
	fee_amount_in_cents: u32,
) -> error::Result<Deposit> {
	let id = uuid::Uuid::now_v7();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Deposit,
		r#"
        INSERT INTO deposit
            (id, account_id, deposit_id, status, deposit_amount_in_cents, payout_amount_in_cents, fee_amount_in_cents, created_at, updated_at)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $8)
        RETURNING
            id as "id!: uuid::Uuid",
            account_id  as "account_id!: uuid::Uuid",
            deposit_id  as "deposit_id!: uuid::Uuid",
			status as "status: DepositStatus",
			deposit_amount_in_cents as "deposit_amount_in_cents!: u32",
			payout_amount_in_cents  as "payout_amount_in_cents!: u32",
			fee_amount_in_cents  as "fee_amount_in_cents!: u32",
            created_at as "created_at!: time::OffsetDateTime",
			updated_at as "updated_at!: time::OffsetDateTime"
        "#,
		id,
		account_id,
		deposit_id,
		status,
		deposit_amount_in_cents,
		payout_amount_in_cents,
		fee_amount_in_cents,
		created_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn update_status_by_deposit_id(pool: impl sqlx::SqliteExecutor<'_>, deposit_id: uuid::Uuid, status: DepositStatus) -> error::Result<Deposit> {
	let updated_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Deposit,
		r#"
        UPDATE
			deposit
		SET
			status = $2,
			updated_at = $3
        WHERE
			deposit_id = $1
        RETURNING
            id as "id!: uuid::Uuid",
            account_id  as "account_id!: uuid::Uuid",
            deposit_id  as "deposit_id!: uuid::Uuid",
			status as "status: DepositStatus",
			deposit_amount_in_cents as "deposit_amount_in_cents!: u32",
			payout_amount_in_cents  as "payout_amount_in_cents!: u32",
			fee_amount_in_cents  as "fee_amount_in_cents!: u32",
            created_at as "created_at!: time::OffsetDateTime",
			updated_at as "updated_at!: time::OffsetDateTime"
        "#,
		deposit_id,
		status,
		updated_at
	)
	.fetch_one(pool)
	.await?)
}

impl From<depix_api::DepositStatus> for DepositStatus {
	#[inline]
	fn from(value: depix_api::DepositStatus) -> Self {
		match value {
			depix_api::DepositStatus::Pending => Self::Pending,
			depix_api::DepositStatus::UnderReview => Self::UnderReview,
			depix_api::DepositStatus::Approved => Self::Approved,
			depix_api::DepositStatus::DepixSent => Self::DepixSent,
			depix_api::DepositStatus::Delayed => Self::Delayed,
			depix_api::DepositStatus::Refunded => Self::Refunded,
			depix_api::DepositStatus::Canceled => Self::Canceled,
			depix_api::DepositStatus::Expired => Self::Expired,
			depix_api::DepositStatus::Error => Self::Error,
			depix_api::DepositStatus::Unknown => Self::Unknown,
			depix_api::DepositStatus::WillRefund => Self::WillRefund,
		}
	}
}
