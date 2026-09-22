use crate::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
	Sqlx(sqlx::error::Error),
	MigrateError(sqlx::migrate::MigrateError),
	Dotenv(dotenvy::Error),
	Var(std::env::VarError, &'static str),
	Io(std::io::Error),
	SystemTime(std::time::SystemTimeError),
	ParseIntError(std::num::ParseIntError),
	Reqwest(reqwest::Error),
	DepixServerError(depix_api::ErrorResponse),
	Json(serde_json::Error),
	FromHexError(hex::FromHexError),
	Jwt(jsonwebtoken::errors::Error),
	Secp256k1(secp256k1::Error),
	InvalidLength,
	Unauthorized,
	InsufficientLimit,
}

impl From<sqlx::error::Error> for Error {
	#[inline(always)]
	fn from(value: sqlx::error::Error) -> Self {
		Self::Sqlx(value)
	}
}

impl From<dotenvy::Error> for Error {
	#[inline(always)]
	fn from(value: dotenvy::Error) -> Self {
		Self::Dotenv(value)
	}
}

impl From<sqlx::migrate::MigrateError> for Error {
	#[inline(always)]
	fn from(value: sqlx::migrate::MigrateError) -> Self {
		Self::MigrateError(value)
	}
}

impl From<std::io::Error> for Error {
	#[inline(always)]
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}

impl From<std::time::SystemTimeError> for Error {
	#[inline(always)]
	fn from(value: std::time::SystemTimeError) -> Self {
		Self::SystemTime(value)
	}
}

impl From<std::num::ParseIntError> for Error {
	#[inline(always)]
	fn from(value: std::num::ParseIntError) -> Self {
		Self::ParseIntError(value)
	}
}

impl From<reqwest::Error> for Error {
	#[inline(always)]
	fn from(value: reqwest::Error) -> Self {
		Self::Reqwest(value)
	}
}

impl From<depix_api::ErrorResponse> for Error {
	#[inline(always)]
	fn from(value: depix_api::ErrorResponse) -> Self {
		Self::DepixServerError(value)
	}
}

impl From<serde_json::Error> for Error {
	#[inline(always)]
	fn from(value: serde_json::Error) -> Self {
		Self::Json(value)
	}
}

impl From<jsonwebtoken::errors::Error> for Error {
	#[inline(always)]
	fn from(value: jsonwebtoken::errors::Error) -> Self {
		Self::Jwt(value)
	}
}

impl From<hex::FromHexError> for Error {
	#[inline(always)]
	fn from(value: hex::FromHexError) -> Self {
		Self::FromHexError(value)
	}
}

impl From<secp256k1::Error> for Error {
	#[inline(always)]
	fn from(value: secp256k1::Error) -> Self {
		Self::Secp256k1(value)
	}
}

impl axum::response::IntoResponse for Error {
	fn into_response(self) -> axum::response::Response {
		match self {
			Self::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized"),
			Self::InsufficientLimit => (axum::http::StatusCode::BAD_REQUEST, "Insufficient limit"),
			error => {
				log::error!("Internal Error {error:?}");

				(axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
			}
		}
		.into_response()
	}
}
