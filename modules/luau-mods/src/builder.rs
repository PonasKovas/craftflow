use anyhow::Context;

use crate::LuauMods;
use std::{
	collections::BTreeMap,
	fs,
	path::{Path, PathBuf},
};

/// Builder for loading the Luau scripts, use to configure.
#[derive(Clone, PartialEq, Debug)]
pub struct LuauModsBuilder {
	load_path: PathBuf,
}

impl LuauModsBuilder {
	/// With default options
	pub fn new() -> Self {
		Self {
			load_path: PathBuf::from("scripts/"),
		}
	}
	/// Sets the filesystem path where to look for the scripts
	pub fn path(&mut self, path: impl AsRef<Path>) -> &mut Self {
		self.load_path = path.as_ref().to_owned();

		self
	}
}

impl Default for LuauModsBuilder {
	fn default() -> Self {
		Self::new()
	}
}

impl LuauModsBuilder {
	/// Loads and initializes the Luau scripts
	pub fn build(&self) -> anyhow::Result<LuauMods> {
		let mut scripts = BTreeMap::new();

		let read_dir = fs::read_dir(&self.load_path)?;

		for entry in read_dir {
			let entry = entry?;

			if entry.file_type()?.is_dir() {
				continue;
			}

			let path = entry.path();

			if path.extension().map(|ex| ex.to_str()).flatten() != Some("luau") {
				continue;
			}

			let contents = fs::read_to_string(&path)?;

			scripts.insert(
				path.file_stem()
					.context("invalid filename")?
					.to_string_lossy()
					.into_owned(),
				contents,
			);
		}

		todo!()
	}
}
