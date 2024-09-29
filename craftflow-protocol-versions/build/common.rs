use std::{fs::read_dir, path::Path};

pub fn snake_to_pascal_case(s: &str) -> String {
	let mut result = String::new();
	let mut capitalize = true;
	for c in s.chars() {
		if c == '_' {
			capitalize = true;
		} else {
			if capitalize {
				result.push(c.to_ascii_uppercase());
				capitalize = false;
			} else {
				result.push(c);
			}
		}
	}
	result
}

pub fn read_dir_sorted(path: impl AsRef<Path>) -> Vec<std::fs::DirEntry> {
	let mut entries = read_dir(path)
		.unwrap()
		.map(|d| d.unwrap())
		.collect::<Vec<_>>();
	entries.sort_by_key(|entry| entry.path());
	entries
}
