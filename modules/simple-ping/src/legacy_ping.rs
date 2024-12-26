use crate::SimplePing;
use craftflow::{
	connection::legacy::{LegacyPing, LegacyPingResponse},
	CraftFlow,
};
use craftflow_protocol_core::common_structures::{text::TextContent, Text};
use std::ops::ControlFlow;

#[craftflow::callback(LegacyPing)]
pub async fn legacy_ping(
	cf: &CraftFlow,
	_conn_id: &mut u64,
) -> ControlFlow<Option<LegacyPingResponse>> {
	let protocol_version = 127; // pretty arbitrary, but its not gonna be compatible with any client anyway
	let online_players = cf.connections().len() as i32; // more or less. (less)
	let max_players = 1000; // todo after implementing max connections
	let description = &cf.modules.get::<SimplePing>().server_description;

	ControlFlow::Break(Some(
		LegacyPingResponse::new(protocol_version, online_players, max_players)
			.set_version(format!("§f§lCraftFlow"))
			.set_description(text_to_legacy(description)),
	))
}

/// Converts a `Text` to a simple string that can be understood by legacy clients
/// Removes anything that cant be converted, and replaces colors with their closest equivalent
pub fn text_to_legacy(text: &Text) -> String {
	// For keeping track of the currently active modifiers
	#[derive(Default, Clone)]
	struct Modifiers {
		obfuscated: Option<bool>,
		bold: Option<bool>,
		strikethrough: Option<bool>,
		underlined: Option<bool>,
		italic: Option<bool>,
		color: Option<char>,
	}

	impl Modifiers {
		// Writes all active modifiers to a String as `§<mod>`
		fn write(&self, output: &mut String) {
			if let Some(color) = self.color {
				output.push('§');
				output.push(color);
			}
			if let Some(true) = self.obfuscated {
				output.push_str("§k");
			}
			if let Some(true) = self.bold {
				output.push_str("§l");
			}
			if let Some(true) = self.strikethrough {
				output.push_str("§m");
			}
			if let Some(true) = self.underlined {
				output.push_str("§n");
			}
			if let Some(true) = self.italic {
				output.push_str("§o");
			}
		}
		// Merges 2 Modifiers. The result is what you would get if you applied them both
		// sequentially.
		fn add(&self, other: &Self) -> Self {
			Self {
				obfuscated: other.obfuscated.or(self.obfuscated),
				bold: other.bold.or(self.bold),
				strikethrough: other.strikethrough.or(self.strikethrough),
				underlined: other.underlined.or(self.underlined),
				italic: other.italic.or(self.italic),
				color: other.color.or(self.color),
			}
		}
	}

	fn to_legacy_inner(this: &Text, result: &mut String, mods: &mut Modifiers) {
		let this = match this {
			Text::String(s) => {
				result.push_str(s);
				return;
			}
			Text::Array(arr) => {
				for child in arr {
					to_legacy_inner(child, result, mods);
				}
				return;
			}
			Text::Object(obj) => obj,
		};

		let new_mods = Modifiers {
			obfuscated: this.obfuscated,
			bold: this.bold,
			strikethrough: this.strikethrough,
			underlined: this.underlined,
			italic: this.italic,
			color: this.color.as_ref().map(|color| color_to_char(&color)),
		};

		// If any modifiers were removed
		if [
			this.obfuscated,
			this.bold,
			this.strikethrough,
			this.underlined,
			this.italic,
		]
		.iter()
		.any(|m| *m == Some(false))
		{
			// Reset and print sum of old and new modifiers
			result.push_str("§r");
			mods.add(&new_mods).write(result);
		} else {
			// Print only new modifiers
			new_mods.write(result);
		}

		*mods = mods.add(&new_mods);

		if let TextContent::Text { text } = &this.content {
			result.push_str(text);
		}

		for child in &this.extra {
			to_legacy_inner(child, result, mods);
		}
	}

	let mut result = String::new();
	let mut mods = Modifiers::default();
	to_legacy_inner(text, &mut result, &mut mods);

	result
}

// returns the closest legacy color code to a given color
// Can be either a full color name or #RRGGBB
fn color_to_char(color: &str) -> char {
	if color.len() == 7 && color.starts_with('#') {
		let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(0);
		let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(0);
		let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(0);

		let mut closest = 0;
		let mut closest_dist = 255 * 255 * 3;
		for (i, (r2, g2, b2)) in [
			(0, 0, 0),
			(0, 0, 170),
			(0, 170, 0),
			(0, 170, 170),
			(170, 0, 0),
			(170, 0, 170),
			(255, 170, 0),
			(170, 170, 170),
			(85, 85, 85),
			(85, 85, 255),
			(85, 255, 85),
			(85, 255, 255),
			(255, 85, 85),
			(255, 85, 255),
			(255, 255, 85),
			(255, 255, 255),
		]
		.iter()
		.enumerate()
		{
			let dist = (r as i32 - r2).pow(2) + (g as i32 - g2).pow(2) + (b as i32 - b2).pow(2);
			if dist < closest_dist {
				closest = i;
				closest_dist = dist;
			}
		}

		"0123456789abcdef".chars().nth(closest).unwrap()
	} else {
		let c = match color.to_lowercase().as_str() {
			"black" => '0',
			"dark_blue" => '1',
			"dark_green" => '2',
			"dark_aqua" => '3',
			"dark_red" => '4',
			"dark_purple" => '5',
			"gold" => '6',
			"gray" => '7',
			"dark_gray" => '8',
			"blue" => '9',
			"green" => 'a',
			"aqua" => 'b',
			"red" => 'c',
			"light_purple" => 'd',
			"yellow" => 'e',
			"white" => 'f',
			_ => '0',
		};

		c
	}
}
