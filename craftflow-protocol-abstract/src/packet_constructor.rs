use crate::ConstructorResult;
use anyhow::Result;
use std::marker::PhantomData;

/// A trait for abstract packet constructor types (for abstract packets that may involve multiple packets)
// we dont allow lifetimes here,
// because we read packets sequentially and only one is available at a time without cloning them.
// and if you have a constructor that needs multiple packets you need to clone and have them 'static
pub trait AbPacketConstructor {
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
		packet: &Self::Direction,
	) -> Result<ConstructorResult<Self::AbPacket, ()>>;
}

/// A constructor to be used when the packet is never gonna use a constructor.
/// That means that the abstract packet will always be constructed from a single concrete packet.
// invariant both over P and D for now, i just dont bother thinking what variance it should be
// for now. Most likely doesn't even matter.
pub struct NoConstructor<P, D>(PhantomData<fn(P, D) -> (P, D)>);

impl<P, D> AbPacketConstructor for NoConstructor<P, D> {
	type AbPacket = P;
	type Direction = D;

	fn next_packet(
		&mut self,
		_packet: &Self::Direction,
	) -> Result<ConstructorResult<Self::AbPacket, ()>> {
		panic!("called next_packet on NoConstructor")
	}
}
