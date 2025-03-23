use super::{MCP, MCPRead, MCPWrite};
use crate::Result;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RestBuffer;

impl MCP for RestBuffer {
	type Data = Vec<u8>;
}
impl<'a> MCPRead<'a> for RestBuffer {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		Ok(input.to_owned())
	}
}

impl MCPWrite for RestBuffer {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		output.extend_from_slice(data);

		data.len()
	}
}
