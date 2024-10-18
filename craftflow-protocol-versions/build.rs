// This build script generates packet enums for every version and state.

#[path = "build/common.rs"]
pub mod common;
#[path = "build/gen_destructure_macro.rs"]
mod gen_destructure_macro;
#[path = "build/gen_impl_trait_macro.rs"]
mod gen_impl_trait_macro;
#[path = "build/generate_packet_enum.rs"]
mod generate_packet_enum;
#[path = "build/generate_state_enum.rs"]
mod generate_state_enum;
#[path = "build/generate_version_enum.rs"]
mod generate_version_enum;
#[path = "build/parse_packet_info.rs"]
mod parse_packet_info;

use std::{
	collections::BTreeMap,
	env,
	fs::{self},
	path::Path,
};

use common::read_dir_sorted;
use gen_destructure_macro::gen_destructure_macro;
use gen_impl_trait_macro::gen_impl_trait_macro;
use generate_packet_enum::generate_packet_enum;
use generate_state_enum::generate_state_enum;
use generate_version_enum::generate_version_enum;
use parse_packet_info::parse_packet_info;

struct Packet {
	// [String] version_name -> Vec<(protocol_version, packet_id)>
	version_variants: BTreeMap<String, Vec<(u32, u32)>>,
}

fn main() {
	for direction in ["c2s", "s2c"] {
		let direction_path = Path::new("src/").join(direction);
		if !direction_path.exists() {
			continue;
		}

		let mut states = Vec::new();

		for state in read_dir_sorted(&direction_path) {
			if !state.file_type().unwrap().is_dir() {
				continue;
			}

			let state_name = state.file_name().into_string().unwrap();

			// [String] packet_name -> Packet
			let mut state_packets = BTreeMap::new();

			for packet in read_dir_sorted(&state.path()) {
				if !packet.file_type().unwrap().is_dir() {
					continue;
				}

				let packet_name = packet.file_name().into_string().unwrap();

				let mut version_variants = Packet {
					version_variants: BTreeMap::new(),
				};

				for version in read_dir_sorted(&packet.path()) {
					if !version.file_type().unwrap().is_dir() {
						continue;
					}

					let version_name = version.file_name().into_string().unwrap();
					let numeric_version = version_name[1..].parse::<u32>().unwrap();

					let packet_info = parse_packet_info(version.path());

					let orig_version = match &packet_info.reexport {
						Some(to) => to,
						None => &version_name,
					};

					version_variants
						.version_variants
						.entry(orig_version.clone())
						.or_insert_with(Vec::new)
						.push((numeric_version, packet_info.packet_id));
				}

				let out_dir_state_path = Path::new(&env::var("OUT_DIR").unwrap())
					.join(direction)
					.join(&state_name);
				fs::create_dir_all(&out_dir_state_path).unwrap();

				fs::write(
					out_dir_state_path.join(format!("{}_enum.rs", packet_name)),
					generate_version_enum(direction, &state_name, &packet_name, &version_variants),
				)
				.unwrap();

				state_packets.insert(packet_name.clone(), version_variants);
			}

			fs::write(
				Path::new(&env::var("OUT_DIR").unwrap())
					.join(direction)
					.join(format!("{}_enum.rs", state_name)),
				generate_packet_enum(direction, &state_name, &state_packets),
			)
			.unwrap();

			states.push(state_name);
		}

		fs::write(
			Path::new(&env::var("OUT_DIR").unwrap()).join(format!("{}_enum.rs", direction)),
			generate_state_enum(direction, &states),
		)
		.unwrap();
	}

	fs::write(
		Path::new(&env::var("OUT_DIR").unwrap()).join("macros.rs"),
		format!("{}\n{}", gen_destructure_macro(), gen_impl_trait_macro()),
	)
	.unwrap();
}
