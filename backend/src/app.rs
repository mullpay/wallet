use crate::*;

#[derive(Clone)]
pub struct AppState {
	pub env: env::Env,
	pub pool: sqlx::sqlite::SqlitePool,
}

impl AppState {
	pub async fn init() -> error::Result<Self> {
		let env = env::Env::init()?;
		let pool = db::connect(&env.database_url).await?;

		Ok(Self { pool, env })
	}
}
