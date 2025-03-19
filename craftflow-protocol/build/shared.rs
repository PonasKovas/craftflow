use std::{env, path::PathBuf};

pub fn package_dir() -> PathBuf {
	PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
		.canonicalize()
		.unwrap()
}

pub fn out_dir() -> PathBuf {
	PathBuf::from(env::var("OUT_DIR").unwrap())
		.canonicalize()
		.unwrap()
}

pub fn snake_to_pascal_case(snake: &str) -> String {
	snake
		.split("_")
		.map(|word| {
			let mut chars = word.chars();

			chars
				.next()
				.map(|c| c.to_uppercase().collect::<String>() + chars.as_str())
				.unwrap_or(String::new())
		})
		.collect()
}

pub fn versions_pattern(versions: &[u32]) -> String {
	versions
		.iter()
		.map(ToString::to_string)
		.collect::<Vec<_>>()
		.join("|")
}
