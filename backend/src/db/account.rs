use crate::*;

#[derive(Debug, Clone)]
pub struct Account {
	pub id: uuid::Uuid,
	pub public_key: crypto::PublicKey,
	pub eulen_id: Option<String>,
	pub created_at: time::OffsetDateTime,
	pub updated_at: time::OffsetDateTime,
}

pub async fn insert_account(pool: impl sqlx::SqliteExecutor<'_>, public_key: &crypto::PublicKey, eulen_id: Option<&str>) -> error::Result<Account> {
	let id = uuid::Uuid::now_v7();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Account,
		r#"
        INSERT INTO account
            (id, public_key, eulen_id, created_at)
        VALUES
            ($1, $2, $3, $4)
        RETURNING
            id as "id!: uuid::Uuid",
            public_key as "public_key!: crypto::PublicKey",
            eulen_id,
            created_at as "created_at!: time::OffsetDateTime",
            updated_at as "updated_at!: time::OffsetDateTime"
        "#,
		id,
		public_key,
		eulen_id,
		created_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_or_insert_account(pool: impl sqlx::SqliteExecutor<'_>, public_key: &crypto::PublicKey) -> error::Result<Account> {
	let id = uuid::Uuid::now_v7();
	let created_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Account,
		r#"
        INSERT INTO account
            (id, public_key, created_at, updated_at)
        VALUES
            ($1, $2, $3, $3)
        ON CONFLICT
            (public_key)
        DO UPDATE SET
            public_key = EXCLUDED.public_key
        RETURNING
            id as "id!: uuid::Uuid",
            public_key as "public_key!: crypto::PublicKey",
            eulen_id,
            created_at as "created_at!: time::OffsetDateTime",
            updated_at as "updated_at!: time::OffsetDateTime"
        "#,
		id,
		public_key,
		created_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn update_eulen_id(pool: impl sqlx::SqliteExecutor<'_>, id: uuid::Uuid, eulen_id: Option<&str>) -> error::Result<Account> {
	let updated_at = time::OffsetDateTime::now_utc();

	Ok(sqlx::query_as!(
		Account,
		r#"
        UPDATE
            account
        SET
            eulen_id = $2,
            updated_at = $3
        WHERE
            id = $1
        RETURNING
            id as "id!: uuid::Uuid",
            public_key as "public_key!: crypto::PublicKey",
            eulen_id,
            created_at as "created_at!: time::OffsetDateTime",
            updated_at as "updated_at!: time::OffsetDateTime"
        "#,
		id,
		eulen_id,
		updated_at
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_account(pool: impl sqlx::SqliteExecutor<'_>, id: &uuid::Uuid) -> error::Result<Account> {
	Ok(sqlx::query_as!(
		Account,
		r#"
        SELECT
            id as "id!: uuid::Uuid",
            public_key as "public_key!: crypto::PublicKey",
            eulen_id,
            created_at as "created_at!: time::OffsetDateTime",
            updated_at as "updated_at!: time::OffsetDateTime"
        FROM
            account
        WHERE
            id = $1
        "#,
		id,
	)
	.fetch_one(pool)
	.await?)
}
