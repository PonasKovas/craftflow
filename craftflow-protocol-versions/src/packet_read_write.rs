use craftflow_protocol_core::Result;
use std::io::Write;

/// Packets that can be parsed from a byte slice given a specific protocol version.
pub trait PacketRead<'a> {
	/// Reads and parses the packet, returning the remaining data (if any) together with the parsed packet.
	fn read_packet(input: &'a [u8], protocol_version: u32) -> Result<(&'a [u8], Self)>
	where
		Self: Sized;
}

/// Packets that can be written to a byte stream. Checks if the packet is valid for the given protocol version.
pub trait PacketWrite {
	/// Writes the packet and returns the number of bytes written
	fn write_packet(&self, output: &mut impl Write, protocol_version: u32) -> Result<usize>;
}

/// MCPRead but generalised for protocol versions
pub trait MCPReadVersioned<'a> {
	/// Reads and parses the data, returning the remaining data (if any) together with the parsed data.
	fn read_versioned(input: &'a [u8], protocol_version: u32) -> Result<(&'a [u8], Self)>
	where
		Self: Sized;
}

/// MCPWrite but generalised for protocol versions
pub trait MCPWriteVersioned {
	/// Writes the data and returns the number of bytes written
	fn write_versioned(&self, output: &mut impl Write, protocol_version: u32) -> Result<usize>;
}
