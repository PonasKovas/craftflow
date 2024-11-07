use crate::ConstructorResult;
use anyhow::Result;
use std::marker::PhantomData;

/// A trait for abstract packet constructor types (for abstract packets that may involve multiple packets)
pub trait AbPacketConstructor<'a> {
	/// The abstract packet type that this type constructs
	type AbPacket;
	/// The direction of the packet
	///
	/// This must be either `craftflow_protocol_versions::C2S` or `craftflow_protocol_versions::S2C`
	type Direction;

	/// Feeds the next packet to the constructor. If the constructor is done, the abstract packet is returned.
	///
	/// Feeding more packets after it was done should result in a panic.
	fn next_packet(
		&mut self,
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self::AbPacket, ()>>;
}

/// A constructor to be used when the packet is never gonna use a constructor.
/// That means that the abstract packet will always be constructed from a single concrete packet.
pub struct NoConstructor<P, D>(PhantomData<fn(P, D)>);

impl<'a, P: 'static, D: 'static> AbPacketConstructor<'a> for NoConstructor<P, D> {
	type AbPacket = P;
	type Direction = D;

	fn next_packet(
		&mut self,
		_packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self::AbPacket, ()>> {
		Ok(ConstructorResult::Ignore)
	}
}
