use crate::packets_toml::PacketsToml;
use std::env;

pub fn generate(pkts_toml: &PacketsToml) -> String {
	let list: Vec<_> = pkts_toml
		.versions
		.iter()
		.filter_map(|&version| {
			// resolve alias if its an alias
			let version = pkts_toml.version_aliases.get(&version).unwrap_or(&version);

			let enabled = env::var(format!("CARGO_FEATURE_NO_V{}", version)).is_err();

			enabled.then(|| format!("{version}"))
		})
		.collect::<Vec<_>>();

	let list_str = list.join(", ");
	let len = list.len();

	format!(
		"/// A list of all supported protocol versions
		pub const SUPPORTED_VERSIONS: [u32; {len}] = [{list_str}];"
	)
}
