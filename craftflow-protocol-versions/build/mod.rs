use gen::generate;
use json_defs::{ProtocolFile, VersionFile};
use proc_macro2::TokenStream;
use std::{
	collections::HashMap,
	env,
	error::Error,
	fs,
	io::Write,
	path::Path,
	process::{Command, Stdio},
};

use crate::VERSIONS;

mod gen;
mod git;
mod json_defs;

const CACHE_DIR: &str = ".cache/minecraft-data/";

pub fn main() -> Result<(), Box<dyn Error>> {
	let repo_path = Path::new(&env::var("CARGO_MANIFEST_DIR")?).join(CACHE_DIR);

	git::prepare_git_repo(&repo_path)?;

	let versions_dir = repo_path.join("data/pc");

	// Create a structure mapping protocol versions to protocol.json files if they exist
	let mut versions = HashMap::new();
	for version in fs::read_dir(versions_dir)? {
		let version = version?;

		let protocol: ProtocolFile = match fs::read_to_string(version.path().join("protocol.json"))
		{
			Ok(p) => serde_json::from_str(&p)?,
			Err(_) => continue, // skip versions that dont change the protocol
		};

		let version: VersionFile =
			serde_json::from_str(&fs::read_to_string(version.path().join("version.json"))?)?;

		versions.insert(version.version, protocol);
	}

	// prepare a structure that groups identical protocols to their JSON definitions
	let mut supported_versions = VERSIONS.to_owned();
	supported_versions.sort();
	let mut last_defined_version = None;
	let mut version_defs = HashMap::new();
	for version in supported_versions {
		match versions.get(&version) {
			Some(protocol) => {
				last_defined_version = Some(version);
				version_defs.insert(version, protocol.clone());
			}
			None => {
				version_defs.insert(
					version,
					versions[&last_defined_version
						.expect("the earliest supported version must have a protocol definition")]
						.clone(),
				);
			}
		}
	}

	let rust_code = generate(version_defs)?;
	let path = Path::new(&env::var("OUT_DIR")?).join(format!("protocol_defs.rs"));
	write(rust_code, path)?;

	Ok(())
}

// Writes stream to the given path
fn write(stream: TokenStream, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
	let mut rustfmt = Command::new("rustfmt")
		.arg("--edition")
		.arg("2021")
		.arg("--config")
		.arg("max_width=10000")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()?;

	{
		let stdin = rustfmt.stdin.as_mut().unwrap();
		stdin.write_all(format!("{stream}").as_bytes())?;
	}

	let output = rustfmt.wait_with_output()?;

	let code = if output.status.success() {
		String::from_utf8(output.stdout)?
	} else {
		// if rustfmt not available or errored due to syntax errors just write unformatted stream
		format!("{stream}")
	};

	fs::write(path, code)?;

	Ok(())
}
