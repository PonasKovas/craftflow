/// Allows to the structure the packet to more general enums, essentially hiding from the type system
/// the actual packet, the state, or even the version.
pub trait Packet {
	type Direction;
	type Version;
	type State;

	/// Converts the packet into the enum containing all packets of this state.
	fn into_state_enum(self) -> Self::State;
	/// Converts the packet into the enum containing all states of this version.
	fn into_version_enum(self) -> Self::Version;
	/// Converts the packet into the enum containing all versions of this direction.
	fn into_direction_enum(self) -> Self::Direction;
}

/// Marks all the versions of the protocol in which this packet is valid.
pub trait PacketVersion {
	const VERSIONS: &'static [u32];
}

/// A trait that marks the equivalency and equality of packets. It's used when a packet doesn't change the structure
/// between a protocol version change. Use the `eqv_packet()` method to easily convert from any packet to another.
pub trait EqvPacket<P>
where
	Self: Sized,
{
	/// Converts the packet into the first equivalent packet.
	fn into_eqv_packet(self) -> P;
	/// Converts the first equivalent packet into the this packet.
	fn from_eqv_packet(p: P) -> Self;
	/// Converts this packet into any other packet that is equivalent to the same packet.
	fn eqv_packet<O: EqvPacket<P>>(self) -> O {
		O::from_eqv_packet(self.into_eqv_packet())
	}
}

// these are generated enums from the build.rs
include!(concat!(env!("OUT_DIR"), "/c2s.rs"));
include!(concat!(env!("OUT_DIR"), "/s2c.rs"));

// automatically generated mods from the python script below:
pub mod v00005;
pub mod v00047;
pub mod v00107;
pub mod v00109;
pub mod v00110;
pub mod v00210;
pub mod v00315;
pub mod v00335;
pub mod v00338;
pub mod v00340;
pub mod v00393;
pub mod v00401;
pub mod v00404;
pub mod v00477;
pub mod v00490;
pub mod v00498;
pub mod v00573;
pub mod v00735;
pub mod v00751;
pub mod v00755;
pub mod v00756;
pub mod v00757;
pub mod v00758;
pub mod v00759;
pub mod v00760;
pub mod v00761;
pub mod v00762;
pub mod v00763;
pub mod v00764;
pub mod v00765;
