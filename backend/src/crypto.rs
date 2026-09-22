use crate::*;

use std::fmt;
use std::ops;

use rand::TryRng;
use rand::rngs::SysRng;
use sha2::Digest;

pub fn rand32() -> [u8; 32] {
	let mut secret = [0u8; 32];

	SysRng.try_fill_bytes(&mut secret).unwrap();

	secret
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash32([u8; 32]);

#[derive(Clone, Copy)]
pub struct PublicKey(secp256k1::PublicKey);

#[derive(Clone, Copy)]
pub struct Signature(secp256k1::ecdsa::Signature);

pub fn sha256(data: &[u8]) -> Hash32 {
	Hash32(sha2::Sha256::digest(data).as_slice().try_into().unwrap())
}

pub fn encode_jwt<T: serde::ser::Serialize>(secret: &[u8], claims: &T) -> error::Result<String> {
	let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret);
	Ok(jsonwebtoken::encode(&jsonwebtoken::Header::default(), claims, &encoding_key)?)
}

pub fn decode_jwt<T: serde::de::DeserializeOwned>(secret: &[u8], token: &str) -> error::Result<T> {
	let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret);
	Ok(jsonwebtoken::decode::<T>(token, &decoding_key, &jsonwebtoken::Validation::default())?.claims)
}

impl Hash32 {
	#[inline(always)]
	pub fn to_bytes(self) -> [u8; 32] {
		self.0
	}
	#[inline(always)]
	pub fn from_bytes(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}
	pub fn to_hex(&self) -> String {
		hex::encode(self.to_bytes())
	}
	pub fn from_hex(value: &str) -> error::Result<Self> {
		let bytes = hex::decode(value)?;
		Ok(Self::from_bytes(bytes.as_slice().try_into().map_err(|_| error::Error::InvalidLength)?))
	}
}

impl serde::Serialize for Hash32 {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_hex().serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for Hash32 {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let hex = String::deserialize(deserializer)?;

		Ok(Self::from_hex(&hex).map_err(|_| serde::de::Error::custom("invalid hash"))?)
	}
}

impl fmt::Debug for Hash32 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.to_hex())
	}
}

impl ops::Deref for Hash32 {
	type Target = [u8; 32];

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for Hash32 {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl PublicKey {
	#[inline(always)]
	pub fn to_bytes(self) -> [u8; 33] {
		self.0.serialize()
	}
	#[inline(always)]
	pub fn from_bytes(bytes: [u8; 33]) -> error::Result<Self> {
		Ok(Self(secp256k1::PublicKey::from_byte_array_compressed(bytes)?))
	}
	pub fn to_hex(&self) -> String {
		hex::encode(self.to_bytes())
	}
	pub fn from_hex(value: &str) -> error::Result<Self> {
		let bytes = hex::decode(value)?;
		Ok(Self::from_bytes(bytes.as_slice().try_into().map_err(|_| error::Error::InvalidLength)?)?)
	}
	pub fn verify(&self, msg: [u8; 32], signature: &Signature) -> error::Result<()> {
		Ok(self.0.verify(secp256k1::Message::from_digest(msg), &signature.0)?)
	}
}

impl serde::Serialize for PublicKey {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_hex().serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for PublicKey {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let hex = String::deserialize(deserializer)?;

		Ok(Self::from_hex(&hex).map_err(|_| serde::de::Error::custom("invalid hash"))?)
	}
}

impl fmt::Debug for PublicKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.to_hex())
	}
}

impl sqlx::Type<sqlx::Sqlite> for PublicKey {
	#[inline(always)]
	fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
		<Vec<u8> as sqlx::Type<sqlx::Sqlite>>::type_info()
	}
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for PublicKey {
	fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
		let value = self.to_bytes().to_vec();

		<Vec<u8> as sqlx::Encode<sqlx::Sqlite>>::encode(value, args)
	}
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for PublicKey {
	fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
		let bytes = <Vec<u8> as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
		let value = bytes.as_slice().try_into().map_err(|_| "invalid hash length (expected 32 bytes)")?;

		Ok(Self::from_bytes(value).map_err(|_| "Invalid public key bytes")?)
	}
}

impl Signature {
	#[inline(always)]
	pub fn to_bytes(self) -> [u8; 64] {
		self.0.serialize_compact()
	}
	#[inline(always)]
	pub fn from_bytes(bytes: [u8; 64]) -> error::Result<Self> {
		Ok(Self(secp256k1::ecdsa::Signature::from_compact(&bytes)?))
	}
	pub fn to_hex(&self) -> String {
		hex::encode(self.to_bytes())
	}
	pub fn from_hex(value: &str) -> error::Result<Self> {
		let bytes = hex::decode(value)?;

		Ok(Self::from_bytes(bytes.as_slice().try_into().map_err(|_| error::Error::InvalidLength)?)?)
	}
}

impl serde::Serialize for Signature {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_hex().serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for Signature {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let hex = String::deserialize(deserializer)?;

		Ok(Self::from_hex(&hex).map_err(|_| serde::de::Error::custom("invalid hash"))?)
	}
}

impl fmt::Debug for Signature {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.to_hex())
	}
}
