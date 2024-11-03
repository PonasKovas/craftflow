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
