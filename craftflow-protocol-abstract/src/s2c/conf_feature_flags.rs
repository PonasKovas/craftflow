use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::Array;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{feature_flags::v00767::FeatureFlagsV00764, FeatureFlags},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use std::iter::{once, Once};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfFeatureFlags {
	pub vanilla: bool,
	pub bundle: bool,
	pub trade_rebalance: bool,
}

impl AbPacketWrite for AbConfFeatureFlags {
	type Direction = S2C;
	type Iter = Once<Self::Direction>;

	fn convert(self, protocol_version: u32) -> Result<WriteResult<Self::Iter>> {
		let pkt = match protocol_version {
			764.. => FeatureFlagsV00764 {
				features: Array::new({
					let mut arr = Vec::new();
					if self.vanilla {
						arr.push("minecraft:vanilla".to_string());
					}
					if self.bundle {
						arr.push("minecraft:bundle".to_string());
					}
					if self.trade_rebalance {
						arr.push("minecraft:trade_rebalance".to_string());
					}
					arr
				}),
			}
			.into_state_enum(),
			_ => return Ok(WriteResult::Unsupported),
		};

		Ok(WriteResult::Success(once(pkt)))
	}
}

impl AbPacketNew for AbConfFeatureFlags {
	type Direction = S2C;
	type Constructor = NoConstructor<Self, S2C>;

	fn construct(
		packet: Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor, Self::Direction>> {
		Ok(match packet {
			S2C::Configuration(Configuration::FeatureFlags(FeatureFlags::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					vanilla: pkt.features.data.iter().any(|x| x == "minecraft:vanilla"),
					bundle: pkt.features.data.iter().any(|x| x == "minecraft:bundle"),
					trade_rebalance: pkt
						.features
						.data
						.iter()
						.any(|x| x == "minecraft:trade_rebalance"),
				})
			}
			_ => ConstructorResult::Ignore(packet),
		})
	}
}
