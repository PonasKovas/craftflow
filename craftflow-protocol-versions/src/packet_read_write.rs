use craftflow_protocol_core::Result;
use std::io::Write;

/// Packets that can be parsed from a byte slice given a specific protocol version.
pub trait PacketRead<'a> {
	/// Reads and parses the packet, returning the remaining data (if any) together with the parsed packet.
	fn read_packet(input: &'a mut [u8], protocol_version: u32) -> Result<(&'a mut [u8], Self)>
	where
		Self: Sized;
}

/// Packets that can be written to a byte stream. Checks if the packet is valid for the given protocol version.
pub trait PacketWrite {
	/// Writes the packet and returns the number of bytes written
	fn write_packet(&self, output: &mut impl Write, protocol_version: u32) -> Result<usize>;
}
