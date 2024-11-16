use crate::{AbPacketNew, AbPacketWrite, ConstructorResult, NoConstructor, State, WriteResult};
use anyhow::Result;
use craftflow_protocol_core::datatypes::Array;
use craftflow_protocol_versions::{
	s2c::{
		configuration::{feature_flags::v00764::FeatureFlagsV00764, FeatureFlags},
		Configuration,
	},
	IntoStateEnum, S2C,
};
use shallowclone::{MakeOwned, ShallowClone};
use std::iter::{once, Once};

#[derive(ShallowClone, MakeOwned, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AbConfFeatureFlags {
	pub vanilla: bool,
	pub bundle: bool,
	pub trade_rebalance: bool,
}

impl<'a> AbPacketWrite<'a> for AbConfFeatureFlags {
	type Direction = S2C<'a>;
	type Iter = Once<Self::Direction>;

	fn convert(&'a self, protocol_version: u32, state: State) -> Result<WriteResult<Self::Iter>> {
		if state != State::Configuration {
			return Ok(WriteResult::Unsupported);
		}

		let pkt = match protocol_version {
			764.. => FeatureFlagsV00764 {
				features: Array::from({
					let mut arr = Vec::new();
					if self.vanilla {
						arr.push("minecraft:vanilla".into());
					}
					if self.bundle {
						arr.push("minecraft:bundle".into());
					}
					if self.trade_rebalance {
						arr.push("minecraft:trade_rebalance".into());
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

impl<'a> AbPacketNew<'a> for AbConfFeatureFlags {
	type Direction = S2C<'a>;
	type Constructor = NoConstructor<AbConfFeatureFlags>;

	fn construct(
		packet: &'a Self::Direction,
	) -> Result<ConstructorResult<Self, Self::Constructor>> {
		Ok(match packet {
			S2C::Configuration(Configuration::FeatureFlags(FeatureFlags::V00764(pkt))) => {
				ConstructorResult::Done(Self {
					vanilla: pkt.features.iter().any(|x| x == "minecraft:vanilla"),
					bundle: pkt.features.iter().any(|x| x == "minecraft:bundle"),
					trade_rebalance: pkt
						.features
						.iter()
						.any(|x| x == "minecraft:trade_rebalance"),
				})
			}
			_ => ConstructorResult::Ignore,
		})
	}
}
