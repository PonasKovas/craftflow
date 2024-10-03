use crate::ConstructorResult;
use anyhow::Result;
use std::marker::PhantomData;

/// A trait for abstract packet constructor types (for abstract packets that may involve multiple packets)
pub trait AbPacketConstructor {
	/// The abstract packet type that this type constructs
	type AbPacket;
	/// The direction of the packet
	///
	/// This must be either `craftflow_protocol_versions::C2S` or `craftflow_protocol_versions::S2C`
	type Direction;

	fn next_packet(
		// Unfortunately due to rustc we must restrict this to Boxed types, because have
		// have to choose between this or not allowing Box<dyn> at all. We can't return Self
		// if it's not Sized (which dyn is not)
		//
		// This has resulted in the signature of this function becoming an actual abomination
		self: Box<Self>,
		packet: Self::Direction,
	) -> Result<
		ConstructorResult<
			Self::AbPacket,
			Box<
				dyn AbPacketConstructor<AbPacket = Self::AbPacket, Direction = Self::Direction>
					+ Send
					+ Sync,
			>,
			(
				Box<
					dyn AbPacketConstructor<AbPacket = Self::AbPacket, Direction = Self::Direction>
						+ Send
						+ Sync,
				>,
				Self::Direction,
			),
		>,
	>;
}

/// A constructor to be used when the packet is never gonna use a constructor.
/// That means that the abstract packet will always be constructed from a single concrete packet.
pub struct NoConstructor<P, D>(PhantomData<fn(P, D)>);

impl<P: 'static, D: 'static> AbPacketConstructor for NoConstructor<P, D> {
	type AbPacket = P;
	type Direction = D;

	fn next_packet(
		self: Box<Self>,
		packet: Self::Direction,
	) -> Result<
		ConstructorResult<
			Self::AbPacket,
			Box<
				dyn AbPacketConstructor<AbPacket = Self::AbPacket, Direction = Self::Direction>
					+ Send
					+ Sync,
			>,
			(
				Box<
					dyn AbPacketConstructor<AbPacket = Self::AbPacket, Direction = Self::Direction>
						+ Send
						+ Sync,
				>,
				Self::Direction,
			),
		>,
	> {
		Ok(ConstructorResult::Ignore((self, packet)))
	}
}
