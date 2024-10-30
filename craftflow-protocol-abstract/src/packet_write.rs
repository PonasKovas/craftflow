use crate::{State, WriteResult};
use anyhow::Result;

/// A trait that allows an abstract packet to be written in a specific protocol version
pub trait AbPacketWrite {
	/// The direction of the packet
	///
	/// This must be either `craftflow_protocol_versions::C2S` or `craftflow_protocol_versions::S2C`
	type Direction;
	/// The return type of the `convert` method. Must be an iterator of concrete packets.
	type Iter: Iterator<Item = Self::Direction>;

	/// Given a protocol version, converts the abstract packet to one or multiple concrete packets.
	fn convert(self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>>;
}
