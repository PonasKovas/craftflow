use std::marker::PhantomData;

use crate::ConstructorResult;
use craftflow_protocol_core::Result;

/// A trait for abstract packet constructor types (for abstract packets that may involve multiple packets)
pub trait AbPacketConstructor {
	/// The abstract packet type that this type constructs
	type AbPacket;
	/// The direction of the packet
	///
	/// This must be either `craftflow_protocol_versions::C2S` or `craftflow_protocol_versions::S2C`
	type Direction;

	fn next_packet(
		self,
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self::AbPacket, Self, (Self, Self::Direction)>>
	where
		Self: Sized;
}

/// A constructor to be used when the packet is never gonna use a constructor.
/// That means that the abstract packet will always be constructed from a single concrete packet.
pub struct NoConstructor<P, D>(PhantomData<fn(P, D)>);

impl<P, D> AbPacketConstructor for NoConstructor<P, D> {
	type AbPacket = P;
	type Direction = D;

	fn next_packet(
		self,
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self::AbPacket, Self, (Self, Self::Direction)>> {
		Ok(ConstructorResult::Ignore((self, packet)))
	}
}
