use crate::ConstructorResult;
use anyhow::Result;
use std::marker::PhantomData;

/// A trait for abstract packet constructor types (for abstract packets that may involve multiple packets)
// we dont allow lifetimes here,
// because we read packets sequentially and only one is available at a time without cloning them.
// and if you have a constructor that needs multiple packets you need to clone and have them 'static
pub trait AbPacketConstructor: 'static {
	/// The abstract packet type that this type constructs
	type AbPacket;

	/// Feeds the next packet to the constructor. If the constructor is done, the abstract packet is returned.
	///
	/// Feeding more packets after it was done should result in a panic.
	fn next_packet(
		&mut self,
		packet: ConcretePacket<'_>,
	) -> Result<ConstructorResult<Self::AbPacket, ()>>;
}

/// Only used internally for abstract packet constructors.
/// This is because:
/// - we need to accept concrete packets with any lifetime in next_packet
/// BUT
/// - the trait itself must not have any lifetimes (must be 'static)
///
/// - so logical thing to do would be to have a lifetime generic in the Direction associated type
/// BUT
/// - we can't have generics in associated types if we want the trait to be object safe
///
/// - so the only solution here is to not have the Direction associated type at all
///   and accept both variants in the constructor, and then match on them.
pub enum ConcretePacket<'a> {
	C2S(&'a craftflow_protocol_versions::C2S<'a>),
	S2C(&'a craftflow_protocol_versions::S2C<'a>),
}

impl<'a> ConcretePacket<'a> {
	pub fn c2s(self) -> &'a craftflow_protocol_versions::C2S<'a> {
		match self {
			ConcretePacket::C2S(c2s) => c2s,
			ConcretePacket::S2C(_) => panic!("expected C2S packet"),
		}
	}
	pub fn s2c(self) -> &'a craftflow_protocol_versions::S2C<'a> {
		match self {
			ConcretePacket::C2S(_) => panic!("expected S2C packet"),
			ConcretePacket::S2C(s2c) => s2c,
		}
	}
}

/// A constructor to be used when the packet is never gonna use a constructor.
/// That means that the abstract packet will always be constructed from a single concrete packet.
// invariant both over P and D for now, i just dont bother thinking what variance it should be
// for now. Most likely doesn't even matter.
#[derive(Clone, Debug)]
pub struct NoConstructor<P>(PhantomData<fn(P) -> P>);

impl<P: 'static> AbPacketConstructor for NoConstructor<P> {
	type AbPacket = P;

	fn next_packet(
		&mut self,
		_packet: ConcretePacket<'_>,
	) -> Result<ConstructorResult<Self::AbPacket, ()>> {
		panic!("called next_packet on NoConstructor")
	}
}
