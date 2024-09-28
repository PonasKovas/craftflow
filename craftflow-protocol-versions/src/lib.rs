pub trait VersionSpecificPacket {
	type Version;
	type Direction;
	type State;

	const VERSION: u32;

	fn into_state_enum(self) -> Self::State;
	fn into_direction_enum(self) -> Self::Direction;
	fn into_version_enum(self) -> Self::Version;
}

include!(concat!(env!("OUT_DIR"), "/c2s.rs"));
include!(concat!(env!("OUT_DIR"), "/s2c.rs"));

pub mod v00005;
pub mod v00047;
pub mod v00107;
pub mod v00109;
pub mod v00110;
pub mod v00210;
pub mod v00315;
pub mod v00335;
pub mod v00338;
pub mod v00340;
pub mod v00393;
pub mod v00401;
pub mod v00404;
pub mod v00477;
pub mod v00490;
pub mod v00498;
pub mod v00573;
pub mod v00735;
pub mod v00751;
pub mod v00755;
pub mod v00756;
pub mod v00757;
pub mod v00758;
pub mod v00759;
pub mod v00760;
pub mod v00761;
pub mod v00762;
pub mod v00763;
pub mod v00764;
pub mod v00765;
