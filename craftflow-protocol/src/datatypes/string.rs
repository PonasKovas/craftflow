use super::{MCP, MCPRead, MCPWrite, VarInt, advance};
use crate::{Error, Result};
use core::str;
use maxlen::BString;

// Hard length limit for any string. Might need to tweak in the future
const HARD_LIMIT: usize = 1024 * 1024;

impl<'a> MCP for &'a str {
	type Data = Self;
}
impl<'a> MCPRead<'a> for &'a str {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let len = VarInt::mcp_read(input)? as usize;

		if len > HARD_LIMIT {
			return Err(Error::StringTooLong {
				length: len,
				max: HARD_LIMIT,
			});
		}

		if input.len() < len {
			return Err(Error::NotEnoughData(len - input.len()));
		}

		let bytes = advance(input, len);
		let s = match str::from_utf8(bytes) {
			Ok(s) => s,
			Err(_) => {
				return Err(Error::StringInvalidUtf8);
			}
		};

		Ok(s)
	}
}
impl MCPWrite for &str {
	fn mcp_write(data: &&str, output: &mut Vec<u8>) -> usize {
		let prefix_len = VarInt::mcp_write(&(data.len() as i32), output);
		output.extend_from_slice(data.as_bytes());

		prefix_len + data.len()
	}
}

impl MCP for String {
	type Data = Self;
}
impl<'a> MCPRead<'a> for String {
	fn mcp_read(input: &mut &[u8]) -> Result<Self> {
		<&str>::mcp_read(input).map(|s| s.to_owned())
	}
}
impl MCPWrite for String {
	fn mcp_write(data: &Self, output: &mut Vec<u8>) -> usize {
		<&str>::mcp_write(&&**data, output)
	}
}

impl<const MAX: usize> MCP for BString<MAX> {
	type Data = Self;
}
impl<'a, const MAX: usize> MCPRead<'a> for BString<MAX> {
	fn mcp_read(input: &mut &[u8]) -> Result<Self> {
		let s = String::mcp_read(input)?;
		let len = s.len();

		BString::from_string(s).map_err(|_| Error::StringTooLong {
			length: len,
			max: MAX,
		})
	}
}
impl<const MAX: usize> MCPWrite for BString<MAX> {
	fn mcp_write(data: &Self, output: &mut Vec<u8>) -> usize {
		<&str>::mcp_write(&&***data, output)
	}
}
