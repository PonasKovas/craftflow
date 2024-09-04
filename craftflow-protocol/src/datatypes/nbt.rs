use super::Nbt;
use crate::MCPReadable;

impl MCPReadable for Nbt {
	fn read(source: &mut impl std::io::Read) -> anyhow::Result<Self>
	where
		Self: Sized,
	{
		todo!()
	}
}
