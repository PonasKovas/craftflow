// This build script generates enums for states, packets and versions (3 nested enums)
// implements the following traits:
// - MCPReadVersioned/MCPWriteVersioned for version enums
// - PacketRead/PacketWrite for packet enums
// - Conversion traits (IntoStateEnum, IntoPacketEnum, IntoVersionEnum) for all of the enums AND
//   the packets themselves
//
// Additionally, it also generates some code to add the types to the source tree.

#[path = "build/common.rs"]
pub mod common;
#[path = "build/gen_conversion.rs"]
mod gen_conversion;
#[path = "build/gen_destructure_macro.rs"]
mod gen_destructure_macro;
#[path = "build/gen_enum.rs"]
mod gen_enum;
#[path = "build/gen_impl_trait_macro.rs"]
mod gen_impl_trait_macro;
#[path = "build/gen_mcp_packet.rs"]
mod gen_mcp_packet;
#[path = "build/gen_mcp_versioned.rs"]
mod gen_mcp_versioned;
#[path = "build/gen_types_code.rs"]
mod gen_types_code;
#[path = "build/parse_packet_info.rs"]
mod parse_packet_info;

use std::{
	env,
	fs::{self},
	path::{Path, PathBuf},
};

use gen_destructure_macro::gen_destructure_macro;
use gen_enum::Variant;
use gen_impl_trait_macro::gen_impl_trait_macro;
use gen_mcp_packet::gen_mcp_packet_impls;
use gen_mcp_versioned::gen_mcp_versioned;
use gen_types_code::gen_types_code;
use parse_packet_info::{
	parse_packets, Direction, Generics, PacketName, PacketType, Packets, State, States, Versions,
};

fn main() {
	let packets = parse_packets();

	let out = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();

	let mut root_code = format!("use shallowclone::ShallowClone;\n");
	for (direction, (dir_generics, states)) in &packets {
		let direction_enum_variants = gen_direction(&out, (direction, dir_generics), states);

		root_code += &format!("pub mod {};\n", direction.mod_name());
		root_code += &gen_enum::gen_enum(&direction.enum_name(), &direction_enum_variants);
		root_code += &gen_conversion::for_direction((direction, dir_generics));
	}

	root_code += &gen_impl_trait_macro(&packets);
	root_code += &gen_destructure_macro(&packets);

	root_code += &gen_types_code();

	fs::write(&out.join("generated.rs"), root_code).unwrap();
}

fn gen_direction(
	out: &PathBuf,
	direction: (&Direction, &Generics),
	states: &States,
) -> Vec<Variant> {
	let mut enum_variants = Vec::new();
	let mut code = format!("use shallowclone::ShallowClone;\n");

	for (state, (st_generics, packets)) in states {
		let state_enum_variants = gen_state(&out, direction, (state, st_generics), packets);

		enum_variants.push(Variant {
			name: state.enum_name(),
			value_path: format!(
				"crate::{dir}::{st}",
				dir = direction.0.mod_name(),
				st = state.enum_name(),
			),
			value_generics: st_generics.clone(),
		});

		code += &format!("pub mod {};\n", state.mod_name());
		code += &gen_enum::gen_enum(&state.enum_name(), &state_enum_variants);
		code += &gen_conversion::for_state(direction, (state, st_generics));
	}

	fs::write(&out.join(direction.0.mod_name()).join("mod.rs"), code).unwrap();

	enum_variants
}

fn gen_state(
	out: &PathBuf,
	direction: (&Direction, &Generics),
	state: (&State, &Generics),
	packets: &Packets,
) -> Vec<Variant> {
	let mut enum_variants = Vec::new();
	let mut code = format!("use shallowclone::ShallowClone;\n");

	for (packet, (pkt_generics, versions)) in packets {
		let version_enum_variants =
			gen_packet(&out, direction, state, (packet, pkt_generics), versions);

		enum_variants.push(Variant {
			name: packet.enum_name(),
			value_path: format!(
				"crate::{dir}::{st}::{pkt}",
				dir = direction.0.mod_name(),
				st = state.0.mod_name(),
				pkt = packet.enum_name(),
			),
			value_generics: pkt_generics.clone(),
		});

		code += &format!("pub mod {};\n", packet.mod_name());
		code += &gen_enum::gen_enum(&packet.enum_name(), &version_enum_variants);
		code += &gen_conversion::for_packet(direction, state, (packet, pkt_generics));
	}
	code += &gen_mcp_packet_impls(direction.0, state.0, packets);

	fs::write(
		&out.join(direction.0.mod_name())
			.join(&state.0.mod_name())
			.join("mod.rs"),
		code,
	)
	.unwrap();

	enum_variants
}

fn gen_packet(
	out: &PathBuf,
	direction: (&Direction, &Generics),
	state: (&State, &Generics),
	packet: (&PacketName, &Generics),
	versions: &Versions,
) -> Vec<Variant> {
	let mut enum_variants = Vec::new();
	let mut packet_code = format!("#[allow(unused_imports)] use shallowclone::ShallowClone;\n");

	let mut packet_enum_generics = Generics::new();
	for (version, packet_info) in versions {
		// only need to generate anything for defined packets, not re-exports
		let (type_name, generics) = match &packet_info.packet_type {
			PacketType::ReExport { .. } => continue,
			PacketType::Defined {
				type_name,
				generics,
			} => (type_name, generics),
		};
		packet_enum_generics = packet_enum_generics.union(generics);

		// prepare a directory to generate stuff in
		fs::create_dir_all(
			&out.join(direction.0.mod_name())
				.join(&state.0.mod_name())
				.join(&packet.0.mod_name())
				.join(&version.mod_name()),
		)
		.unwrap();

		enum_variants.push(Variant {
			name: version.caps_mod_name(),
			value_path: format!(
				"crate::{dir}::{st}::{pkt}::{v}::{type_name}",
				dir = direction.0.mod_name(),
				st = state.0.mod_name(),
				pkt = packet.0.mod_name(),
				v = version.mod_name(),
			),
			value_generics: generics.clone(),
		});

		let mut version_code = format!(
			"
			#[allow(unused_imports)]
            use craftflow_protocol_core::datatypes::*;
            #[allow(unused_imports)]
            use craftflow_protocol_core::*;
            #[allow(unused_imports)]
            use std::borrow::Cow;
            #[allow(unused_imports)]
            use craftflow_protocol_core::common_structures::*;
            #[allow(unused_imports)]
            use crate::types::{v}::*;
            #[allow(unused_imports)]
            use shallowclone::ShallowClone;
            #[allow(unused_imports)]
            use craftflow_nbt::DynNBT;

            include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{dir}/{st}/{pkt}/{v}/mod.rs\"));
            ",
			dir = direction.0.mod_name(),
			st = state.0.mod_name(),
			pkt = packet.0.mod_name(),
			v = version.mod_name(),
		);
		version_code += &gen_conversion::for_version(direction, state, packet, (version, generics));
		fs::write(
			&out.join(direction.0.mod_name())
				.join(&state.0.mod_name())
				.join(&packet.0.mod_name())
				.join(&version.mod_name())
				.join("mod.rs"),
			version_code,
		)
		.unwrap();

		packet_code += &format!("pub mod {};\n", version.mod_name());
	}

	packet_code += &gen_mcp_versioned(
		direction.0,
		state.0,
		packet.0,
		versions,
		&packet_enum_generics,
	);

	fs::write(
		&out.join(direction.0.mod_name())
			.join(&state.0.mod_name())
			.join(&packet.0.mod_name())
			.join("mod.rs"),
		packet_code,
	)
	.unwrap();

	enum_variants
}
