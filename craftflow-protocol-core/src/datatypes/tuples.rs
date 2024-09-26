use crate::{MCPRead, MCPWrite};
use std::io::Write;

impl<T1: MCPWrite, T2: MCPWrite> MCPWrite for (T1, T2) {
	fn write(&self, output: &mut impl Write) -> crate::Result<usize> {
		self.0.write(output)?;
		self.1.write(output)
	}
}
impl<T1: MCPRead, T2: MCPRead> MCPRead for (T1, T2) {
	fn read(input: &[u8]) -> crate::Result<(&[u8], Self)> {
		let (input, v1) = T1::read(input)?;
		let (input, v2) = T2::read(input)?;

		Ok((input, (v1, v2)))
	}
}

impl<T1: MCPWrite, T2: MCPWrite, T3: MCPWrite> MCPWrite for (T1, T2, T3) {
	fn write(&self, output: &mut impl Write) -> crate::Result<usize> {
		self.0.write(output)?;
		self.1.write(output)?;
		self.2.write(output)
	}
}
impl<T1: MCPRead, T2: MCPRead, T3: MCPRead> MCPRead for (T1, T2, T3) {
	fn read(input: &[u8]) -> crate::Result<(&[u8], Self)> {
		let (input, v1) = T1::read(input)?;
		let (input, v2) = T2::read(input)?;
		let (input, v3) = T3::read(input)?;

		Ok((input, (v1, v2, v3)))
	}
}
