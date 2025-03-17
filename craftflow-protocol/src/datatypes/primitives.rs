use super::advance;
use crate::{MCPRead, MCPWrite, Result};

impl<'a> MCPRead<'a> for bool {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		Ok(u8::mcp_read(input)? != 0)
	}
}
impl MCPWrite for bool {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		output.push(*self as u8);
		1
	}
}

macro_rules! impl_int {
	($($name:ty),+ $(,)?) => {$(
		const _: () = {
			const SIZE: usize = std::mem::size_of::<$name>();
			impl<'a> MCPRead<'a> for $name {
				fn mcp_read(input: &mut &'a[u8]) -> Result<Self> {
					if input.len() < SIZE {
						return Err(crate::Error::NotEnoughData(SIZE - input.len()));
					}

					let b = <[u8; SIZE]>::try_from(advance(input, SIZE)).unwrap();
					let r = Self::from_be_bytes(b);

					Ok(r)
				}
			}
			impl MCPWrite for $name {
				fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
					output.extend_from_slice(&self.to_be_bytes()[..]);

					SIZE
				}
			}
		};
	)+};
}
impl_int!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64);
