use crate::Error;
use crate::Result;
use crate::{MCPRead, MCPWrite};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> MCPRead for Json<T> {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, raw_str) = String::read(input)?;

		let json: T = match serde_json::from_str(&raw_str) {
			Ok(json) => json,
			Err(e) => {
				return Err(Error::InvalidData(format!(
					"Failed to parse JSON Text component: {e}"
				)))
			}
		};

		Ok((input, Self(json)))
	}
}

impl<T: Serialize> MCPWrite for Json<T> {
	fn write(&self, output: &mut impl Write) -> Result<usize> {
		let s = match serde_json::to_string(&self.0) {
			Ok(s) => s,
			Err(e) => {
				return Err(Error::InvalidData(format!(
					"Failed to serialize JSON Text component: {e}"
				)))
			}
		};

		s.write(output)
	}
}
