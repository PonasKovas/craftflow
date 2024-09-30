use craftflow_protocol_core::Result;

/// A trait that allows an abstract packet to be written in a specific protocol version
pub trait AbPacketWrite {
	/// The direction of the packet
	///
	/// This must be either `craftflow_protocol_versions::C2S` or `craftflow_protocol_versions::S2C`
	type Direction;

	/// Given a protocol version, converts the abstract packet to one or multiple concrete packets and calls the `writer`
	/// closure with them. The `writer` closure should handle writing the packet to the stream or whatever else you want.
	fn convert_and_write(
		self,
		protocol_version: u32,
		writer: impl FnMut(Self::Direction) -> Result<()>,
	) -> Result<()>;
}
