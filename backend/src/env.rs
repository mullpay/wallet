use std::env;

use crate::*;

#[derive(Debug, Clone)]
pub struct Env {
	pub database_url: String,
	pub server_addr: String,
	pub server_port: u16,
	pub depix_token: String,
	pub spark_address: String,
	pub jwt_secret: String,
	pub jwt_expiration_seconds: u64,
	pub jwt_challenge_secret: String,
	pub jwt_challenge_expiration_seconds: u64,
	pub webhook_secret: String,
}

impl Env {
	pub fn init() -> error::Result<Self> {
		let _ = dotenvy::dotenv();

		let server_addr = get_env("SERVER_ADDR")?;
		let server_port = get_env("SERVER_PORT")?.parse()?;
		let database_url = get_env("DATABASE_URL")?;

		let depix_token = get_env("DEPIX_TOKEN")?;
		let spark_address = get_env("SPARK_ADDRESS")?;

		let jwt_secret = get_env("JWT_SECRET")?;
		let jwt_expiration_seconds = get_env("JWT_EXPIRATION_SECONDS")?.parse()?;
		let jwt_challenge_secret = get_env("JWT_CHALLENGE_SECRET")?;
		let jwt_challenge_expiration_seconds = get_env("JWT_CHALLENGE_EXPIRATION_SECONDS")?.parse()?;

		let webhook_secret = get_env("WEBHOOK_SECRET")?;

		Ok(Self {
			database_url,
			server_addr,
			server_port,
			depix_token,
			spark_address,
			jwt_secret,
			jwt_expiration_seconds,
			jwt_challenge_secret,
			jwt_challenge_expiration_seconds,
			webhook_secret,
		})
	}
}

fn get_env(key: &'static str) -> error::Result<String> {
	Ok(env::var(key).map_err(|err| error::Error::Var(err, key))?)
}
