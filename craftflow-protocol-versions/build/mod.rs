use gen::generate;
use proc_macro2::TokenStream;
use std::{
	collections::HashMap,
	env, fs,
	io::Write,
	path::Path,
	process::{Command, Stdio},
};

mod gen;
mod git;

const CACHE_DIR: &str = ".cache/minecraft-data/";

pub fn main() {
	let repo_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(CACHE_DIR);

	git::prepare_git_repo(&repo_path);

	let versions_dir = repo_path.join("data/pc");

	// Create a structure mapping protocol versions to protocol.json files if they exist
	let mut versions = HashMap::new();
	for version in fs::read_dir(versions_dir).unwrap() {
		let version = version.unwrap();

		let protocol: serde_json::Value =
			match fs::read_to_string(version.path().join("protocol.json")) {
				Ok(p) => match serde_json::from_str(&p) {
					Ok(p) => p,
					Err(e) => panic!("failed to parse {:?}: {e}", version.path()),
				},
				Err(_) => continue, // skip versions that dont change the protocol
			};

		let version: serde_json::Value = match serde_json::from_str(
			&fs::read_to_string(version.path().join("version.json")).unwrap(),
		) {
			Ok(v) => v,
			Err(e) => panic!("failed to parse {:?}: {e}", version.path()),
		};

		versions.insert(version["version"].as_u64().unwrap() as u32, protocol);
	}

	let rust_code = generate(versions);
	let path = Path::new(&env::var("OUT_DIR").unwrap()).join(format!("protocol_defs.rs"));
	write(rust_code, path);
}

// Writes stream to the given path
fn write(stream: TokenStream, path: impl AsRef<Path>) {
	let mut rustfmt = Command::new("rustfmt")
		.arg("--edition")
		.arg("2021")
		.arg("--config")
		.arg("max_width=10000")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.unwrap();

	{
		let stdin = rustfmt.stdin.as_mut().unwrap();
		stdin.write_all(format!("{stream}").as_bytes()).unwrap();
	}

	let output = rustfmt.wait_with_output().unwrap();

	let code = if output.status.success() {
		String::from_utf8(output.stdout).unwrap()
	} else {
		// if rustfmt not available or errored due to syntax errors just write unformatted stream
		format!("{stream}")
	};

	fs::write(path, code).unwrap();
}
