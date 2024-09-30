use crate::ConstructorResult;
use craftflow_protocol_core::Result;

/// A trait for abstract packet types that allows to try to start constructing them from a concrete packet
pub trait AbPacketNew {
	/// The direction of the packet
	///
	/// This must be either `craftflow_protocol_versions::C2S` or `craftflow_protocol_versions::S2C`
	type Direction;
	/// If this abstract packet may involve multiple concrete packets, this is the type that will handle
	/// the subsequent packets. Otherwise, it should be [`NoConstructor`][crate::NoConstructor].
	type Constructor;

	/// Attempts to construct a new abstract packet from the given concrete packet
	/// If this abstract packed does not involve the given concrete packet, Ignore is returned.
	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>>
	where
		Self: Sized;
}
