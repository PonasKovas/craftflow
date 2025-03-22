use super::{MCP, MCPRead, MCPWrite};
use crate::{Error, Result, limits::DEFAULT_ARRAY_LEN_LIMIT};
use maxlen::BVec;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RestBuffer<const MAX: usize = DEFAULT_ARRAY_LEN_LIMIT>;

impl<const MAX: usize> MCP for RestBuffer<MAX> {
	type Data = BVec<u8, MAX>;
}
impl<'a> MCPRead<'a> for RestBuffer {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self::Data> {
		let bvec = BVec::from_vec(input.to_owned()).map_err(|e| Error::ArrayTooLong {
			length: e.length,
			max: e.maximum,
		})?;

		Ok(bvec)
	}
}

impl MCPWrite for RestBuffer {
	fn mcp_write(data: &Self::Data, output: &mut Vec<u8>) -> usize {
		output.extend_from_slice(data);

		data.len()
	}
}
