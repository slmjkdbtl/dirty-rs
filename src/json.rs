// wengwengweng

use serde::ser;
use serde::de;

use crate::Result;

pub fn encode<D: ser::Serialize>(data: D) -> Result<String> {
	return Ok(serde_json::to_string(&data)?);
}

pub fn decode<D: for<'a> de::Deserialize<'a>>(string: &str) -> Result<D> {
	return Ok(serde_json::from_str(&string)?);
}
