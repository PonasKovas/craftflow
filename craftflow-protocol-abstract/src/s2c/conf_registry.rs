use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_nbt::DynNBT;
use craftflow_protocol_core::datatypes::AnonymousNbt;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{registry_data::v00765::RegistryDataV00764, RegistryData},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use std::{
	iter::{once, Once},
	sync::OnceLock,
};

// Minecraft SLOP
// why tf is this even being sent
// half of it is not even being used, other half could just be handled completely on the server

#[derive(Debug, Clone, PartialEq)]
pub struct AbConfRegistry {
	pub data: DynNBT,
}

impl AbConfRegistry {
	pub fn default() -> Self {
		static DEFAULT: OnceLock<DynNBT> = OnceLock::new();

		let data = DEFAULT
			.get_or_init(|| {
				let json_data = include_str!(concat!(
					env!("CARGO_MANIFEST_DIR"),
					"/assets/default_registry.json"
				));

				serde_json::from_str(json_data).unwrap()
			})
			.clone();

		Self { data }
	}
}

impl AbPacketWrite for AbConfRegistry {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			764.. => RegistryDataV00764 {
				codec: AnonymousNbt { inner: self.data },
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfRegistry {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::RegistryData(RegistryData::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					data: pkt.codec.inner,
				})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_registry() {
		let _packet = AbConfRegistry::default();
	}
}
