use craftflow_protocol_core::Result;
use std::io::Write;

/// MCPRead but generalised for protocol versions
pub trait MCPReadVersioned<'read> {
	/// Reads and parses the data, returning the remaining data (if any) together with the parsed data.
	fn read_versioned(input: &'read [u8], protocol_version: u32) -> Result<(&'read [u8], Self)>
	where
		Self: Sized;
}

/// MCPWrite but generalised for protocol versions
pub trait MCPWriteVersioned {
	/// Writes the data and returns the number of bytes written
	fn write_versioned(&self, output: &mut impl Write, protocol_version: u32) -> Result<usize>;
}
