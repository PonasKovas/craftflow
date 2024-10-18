#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct Tags(pub Array<VarInt, Tag>);

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Ord, Eq)]
pub struct Tag {
	pub tag_name: String,
	pub entries: Array<VarInt, VarInt>,
}

impl MCPWrite for Tags {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		self.0.write(output)
	}
}

impl MCPWrite for Tag {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;
		written_bytes += self.tag_name.write(output)?;
		written_bytes += self.entries.write(output)?;
		Ok(written_bytes)
	}
}

impl MCPRead for Tags {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, tags) = Array::<VarInt, Tag>::read(input)?;
		Ok((input, Self(tags)))
	}
}

impl MCPRead for Tag {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, tag_name) = String::read(input)?;
		let (input, entries) = Array::<VarInt, VarInt>::read(input)?;
		Ok((input, Self { tag_name, entries }))
	}
}
