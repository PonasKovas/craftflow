// This build script generates packet enums for every version and state.

#[path = "build/common.rs"]
pub mod common;
#[path = "build/gen_destructure_macro.rs"]
mod gen_destructure_macro;
#[path = "build/generate_packet_enum.rs"]
mod generate_packet_enum;
#[path = "build/generate_state_enum.rs"]
mod generate_state_enum;
#[path = "build/generate_version_enum.rs"]
mod generate_version_enum;
#[path = "build/parse_packet_info.rs"]
mod parse_packet_info;

use std::{
	collections::{btree_map::Entry, BTreeMap},
	env,
	fs::{self},
	path::Path,
};

use common::read_dir_sorted;
use gen_destructure_macro::gen_destructure_macro;
use generate_packet_enum::generate_packet_enum;
use generate_state_enum::generate_state_enum;
use generate_version_enum::generate_version_enum;
use parse_packet_info::{parse_packet_info, PacketInfo};

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

			// [String] packet_name ->
			// BTreeMap<String, (Vec<u32>, u32)> protocol version variant ->
			// ([protocol versions], packet_id)
			let mut state_packets = BTreeMap::new();

			for packet in read_dir_sorted(&state.path()) {
				if !packet.file_type().unwrap().is_dir() {
					continue;
				}

				let packet_name = packet.file_name().into_string().unwrap();

				// [String] version_name -> (Vec<protocol versions>, [u32] packet id)
				let mut packet_ids = BTreeMap::new();

				// [String] version_name ->
				// Vec<u32> versions that use this variant (the original and all those that are re-exporting it)
				let mut packet_versions = BTreeMap::new();

				for version in read_dir_sorted(&packet.path()) {
					if !version.file_type().unwrap().is_dir() {
						continue;
					}

					let version_name = version.file_name().into_string().unwrap();
					let numeric_version = version_name[1..].parse::<u32>().unwrap();

					let packet_info = parse_packet_info(version.path().join("packet_info"));

					let orig_packet;
					let mut packet_id = None;
					match packet_info {
						PacketInfo::Defined { packet_id: id } => {
							orig_packet = version_name.clone();
							packet_id = Some(id);
						}
						PacketInfo::ReExported { to } => {
							orig_packet = to.clone();
						}
					}

					match packet_versions.entry(orig_packet.clone()) {
						Entry::Vacant(entry) => {
							entry.insert(vec![numeric_version]);
						}
						Entry::Occupied(mut entry) => {
							entry.get_mut().push(numeric_version);
						}
					}

					match packet_ids.entry(orig_packet) {
						Entry::Vacant(entry) => {
							// if packet_id is None, use 80085 as a placeholder
							// it will get replaced when we iterate over the defined version
							entry.insert((vec![numeric_version], packet_id.unwrap_or(80085)));
						}
						Entry::Occupied(mut entry) => {
							entry.get_mut().0.push(numeric_version);
							if let Some(packet_id) = packet_id {
								entry.get_mut().1 = packet_id;
							}
						}
					}
				}

				state_packets.insert(packet_name.clone(), packet_ids);

				let out_dir_state_path = Path::new(&env::var("OUT_DIR").unwrap())
					.join(direction)
					.join(&state_name);
				fs::create_dir_all(&out_dir_state_path).unwrap();
				fs::write(
					out_dir_state_path.join(format!("{}_enum.rs", packet_name)),
					generate_version_enum(direction, &state_name, &packet_name, &packet_versions),
				)
				.unwrap();
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
		Path::new(&env::var("OUT_DIR").unwrap()).join("enum_destructure_macro.rs"),
		gen_destructure_macro(),
	)
	.unwrap();
}
