#![feature(doc_cfg)]

pub mod datatypes;
pub mod protocol;
pub(crate) mod stable_packets;

use anyhow::Result;
use std::io::{Read, Write};

/// Types that can be (de)serialized in the Minecraft network protocol format
pub trait MinecraftProtocol {
	fn read(protocol_version: u32, input: &mut impl Read) -> Result<Self>
	where
		Self: Sized;

	/// Writes the data and returns the number of bytes written
	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize>;
}

/// Types that are packets
pub trait Packet {
	type Direction;

	fn into_packet_enum(self) -> Self::Direction;
}
