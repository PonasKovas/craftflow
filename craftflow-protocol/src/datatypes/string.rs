use super::{VarInt, advance};
use crate::{Error, MCPRead, MCPWrite, Result};
use core::str;
use maxlen::BString;

// Hard length limit for any string. Might need to tweak in the future
const HARD_LIMIT: usize = 1024 * 1024;

impl<'a> MCPRead<'a> for &'a str {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let len = VarInt::mcp_read(input)?.0 as usize;

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
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		let prefix_len = VarInt(self.len() as i32).mcp_write(output);
		output.extend_from_slice(self.as_bytes());

		prefix_len + self.len()
	}
}

impl<'a> MCPRead<'a> for String {
	fn mcp_read(input: &mut &[u8]) -> Result<Self> {
		<&str>::mcp_read(input).map(|s| s.to_owned())
	}
}
impl MCPWrite for String {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		(&**self).mcp_write(output)
	}
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
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		(&***self).mcp_write(output)
	}
}
