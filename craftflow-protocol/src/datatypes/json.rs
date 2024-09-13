use crate::Error;
use crate::MinecraftProtocol;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Json<T>(pub T);

impl<'a, T: Deserialize<'a> + Serialize> MinecraftProtocol<'a> for Json<T> {
	fn read(protocol_version: u32, input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, raw_str) = Cow::<'a, str>::read(protocol_version, input)?;

		let raw_str = match raw_str {
			Cow::Borrowed(s) => s,
			Cow::Owned(_) => unreachable!("Cow<str>::read always returns Borrowed variant"),
		};

		let json = match serde_json::from_str(raw_str) {
			Ok(json) => json,
			Err(e) => {
				return Err(Error::InvalidData(format!(
					"Failed to parse JSON Text component: {e}"
				)))
			}
		};

		Ok((input, Self(json)))
	}
	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
		// A small Write wrapper that counts the written bytes
		struct CountingWrite<'a, W> {
			count: usize,
			output: &'a mut W,
		}
		impl<'a, W: Write> Write for CountingWrite<'a, W> {
			fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
				let count = self.output.write(buf)?;
				self.count += count;
				Ok(count)
			}
			fn flush(&mut self) -> std::io::Result<()> {
				self.output.flush()
			}
		}

		let mut writer = CountingWrite { count: 0, output };

		match serde_json::to_writer(&mut writer, &self.0) {
			Ok(()) => {}
			Err(e) => {
				return Err(Error::InvalidData(format!(
					"Failed to serialize JSON Text component: {e}"
				)))
			}
		}

		Ok(writer.count)
	}
}
