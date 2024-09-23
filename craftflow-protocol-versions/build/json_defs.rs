//! The JSON definitions for parsing the minecraft-data files

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VersionFile {
	pub version: u32,
	// there are more fields but we have no interest in them
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProtocolFile {
	pub version: u32,
}
