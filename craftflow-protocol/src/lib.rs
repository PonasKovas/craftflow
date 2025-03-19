#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

pub mod datatypes;
mod error;
mod generator;

pub use error::{Error, Result};

/// The main internal trait that allows to write data
trait MCPWrite {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize;
}

/// The main internal trait that allows to read data
trait MCPRead<'a>: Sized {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self>;
}

/// Trait for reading a packet in a specific protocol version
pub trait PacketRead<'a>: Sized {
	fn packet_read(input: &mut &'a [u8], protocol: u32) -> Result<Self>;
}

/// Trait for writing a packet
pub trait PacketWrite {
	fn packet_write(&self, output: &mut Vec<u8>) -> usize;
}

/// Allows converting the packet into an enum that abstracts the packet's version.
pub trait IntoPacketEnum {
	type Packet;

	fn into_version_enum(self) -> Self::Packet;
}

/// Allows converting the packet into an enum that abstracts the packet's type.
pub trait IntoStateEnum {
	type State;

	fn into_packet_enum(self) -> Self::State;
}

/// Allows converting the packet into an enum that abstracts the packet's state.
pub trait IntoDirectionEnum {
	type Direction;

	fn into_state_enum(self) -> Self::Direction;
}
