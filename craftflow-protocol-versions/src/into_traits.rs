/// Allows converting the packet into an enum that abstracts the packet's version.
pub trait IntoVersionEnum {
	type Packet;

	fn into_version_enum(self) -> Self::Packet;
}

/// Allows converting the packet into an enum that abstracts the packet's type.
pub trait IntoPacketEnum {
	type State;

	fn into_packet_enum(self) -> Self::State;
}

/// Allows converting the packet into an enum that abstracts the packet's state.
pub trait IntoStateEnum {
	type Direction;

	fn into_state_enum(self) -> Self::Direction;
}
