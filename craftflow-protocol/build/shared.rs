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
				.unwrap_or_default()
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

pub fn group_consecutive(
	iter: impl Iterator<Item = (u32, bool)>,
) -> impl Iterator<Item = (u32, u32, bool)> {
	let mut iter = iter.peekable();
	let mut current = None;

	std::iter::from_fn(move || {
		// Initialize current group if empty
		if current.is_none() {
			let (num, b) = iter.next()?;
			current = Some((num, num, b));
		}

		// Extend group while next element has the same bool
		while let Some((next_num, next_b)) = iter.peek() {
			let (start, _end, b) = current.unwrap();
			if *next_b == b {
				// Extend the current group
				current = Some((start, *next_num, b));
				iter.next(); // Consume the element
			} else {
				break;
			}
		}

		// Return the completed group
		current.take()
	})
}
