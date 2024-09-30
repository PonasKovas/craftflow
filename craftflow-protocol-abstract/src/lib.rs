use craftflow_protocol_core::Result;

mod c2s;
mod s2c;

pub use c2s::AbC2S;
pub use s2c::AbS2C;

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

/// A trait for abstract packet constructor types (for abstract packets that may involve multiple packets)
pub trait AbPacketConstructor: Sized {
	/// The abstract packet type that this type constructs
	type AbPacket;
	/// The direction of the packet
	///
	/// This must be either `craftflow_protocol_versions::C2S` or `craftflow_protocol_versions::S2C`
	type Direction;

	fn next_packet(
		self,
		packet: Self::Direction,
	) -> Result<AbConstrResult<Self::AbPacket, Self, Self::Direction>>;
}

/// A trait for abstract packet types that allows to try to start constructing them from a concrete packet
pub trait AbPacketNew: Sized {
	/// The direction of the packet
	///
	/// This must be either `craftflow_protocol_versions::C2S` or `craftflow_protocol_versions::S2C`
	type Direction;
	/// If this abstract packet may involve multiple concrete packets, this is the type that will handle
	/// the subsequent packets. Otherwise, it can be `()`.
	type Constructor;

	/// Attempts to construct a new abstract packet from the given concrete packet
	/// If this abstract packed does not involve the given concrete packet, Ignore is returned.
	fn construct(
		packet: Self::Direction,
	) -> Result<AbConstrResult<Self, Self::Constructor, Self::Direction>>;
}

/// Returned by an abstract packet constructor to indicate the result of processing a packet
pub enum AbConstrResult<F, C, P> {
	/// The constructor is done and the abstract packet is ready
	Done(F),
	/// The constructor needs more packets to finish
	Continue(C),
	/// The packet was not needed by this constructor
	/// Or there is no constructor that would accept this packet.
	Ignore((C, P)),
}

impl AbPacketConstructor for () {
	type AbPacket = ();
	type Direction = ();

	fn next_packet(
		self,
		_: Self::Direction,
	) -> Result<AbConstrResult<Self::AbPacket, Self, Self::Direction>> {
		panic!()
	}
}
