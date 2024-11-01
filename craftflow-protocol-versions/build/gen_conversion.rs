use crate::{
	common::get_lifetime,
	parse_packet_info::{Direction, HasLifetime, PacketName, State, Version},
};

/// Generates IntoVersionEnum, IntoPacketEnum and IntoStateEnum for a specific packet version
pub fn for_version(
	(direction, dir_lifetime): (&Direction, HasLifetime),
	(state, st_lifetime): (&State, HasLifetime),
	(packet, pkt_lifetime): (&PacketName, HasLifetime),
	(version, v_lifetime): (&Version, HasLifetime),
) -> String {
	let dir_mod = direction.mod_name();
	let state_mod = &state.0;
	let packet_mod = &packet.0;
	let version_mod = version.mod_name();

	let path = format!(
		"crate::{dir_mod}::{state_mod}::{packet_mod}::{version_mod}::{pkt_pascal}{v_caps}",
		pkt_pascal = packet.enum_name(),
		v_caps = version.caps_mod_name(),
	);

	let mut r = String::new();
	r += &gen_impl_block(
		Trait::Version,
		&path,
		v_lifetime,
		&format!(
			"crate::{dir_mod}::{state_mod}::{packet_enum}",
			packet_enum = packet.enum_name()
		),
		pkt_lifetime,
		&version.caps_mod_name(),
		true,
	);
	r += &gen_impl_block(
		Trait::Packet,
		&path,
		pkt_lifetime,
		&format!(
			"crate::{dir_mod}::{state_enum}",
			state_enum = state.enum_name()
		),
		st_lifetime,
		&packet.enum_name(),
		false,
	);
	r += &gen_impl_block(
		Trait::State,
		&path,
		st_lifetime,
		&format!("crate::{dir_enum}", dir_enum = direction.enum_name()),
		dir_lifetime,
		&state.enum_name(),
		false,
	);

	r
}

/// Generates IntoVersionEnum, IntoPacketEnum and IntoStateEnum for a specific packet (version enum)
pub fn for_packet(
	(direction, dir_lifetime): (&Direction, HasLifetime),
	(state, st_lifetime): (&State, HasLifetime),
	(packet, pkt_lifetime): (&PacketName, HasLifetime),
) -> String {
	let dir_mod = direction.mod_name();

	let path = format!(
		"crate::{dir_mod}::{state_mod}::{packet_enum}",
		state_mod = &state.mod_name(),
		packet_enum = packet.enum_name(),
	);

	let mut r = String::new();
	r += &gen_self_impl_block(Trait::Version, &path, pkt_lifetime);
	r += &gen_impl_block(
		Trait::Packet,
		&path,
		st_lifetime,
		&format!(
			"crate::{dir_mod}::{state_enum}",
			state_enum = state.enum_name()
		),
		st_lifetime,
		&packet.enum_name(),
		true,
	);
	r += &gen_impl_block(
		Trait::State,
		&path,
		st_lifetime,
		&format!("crate::{dir_enum}", dir_enum = direction.enum_name()),
		dir_lifetime,
		&state.enum_name(),
		false,
	);
	r
}

/// Generates IntoPacketEnum and IntoStateEnum for a specific state (packet enum)
pub fn for_state(
	(direction, dir_lifetime): (&Direction, HasLifetime),
	(state, st_lifetime): (&State, HasLifetime),
) -> String {
	let path = format!(
		"crate::{dir_mod}::{state_enum}",
		dir_mod = direction.mod_name(),
		state_enum = state.enum_name()
	);

	let super_path = format!("crate::{}", direction.enum_name());

	gen_self_impl_block(Trait::Packet, &path, st_lifetime)
		+ &gen_impl_block(
			Trait::State,
			&path,
			st_lifetime,
			&super_path,
			dir_lifetime,
			&state.enum_name(),
			true,
		)
}

/// Generates IntoStateEnum for a specific direction (state enum)
pub fn for_direction((direction, dir_lifetime): (&Direction, HasLifetime)) -> String {
	let path = format!("crate::{dir_enum}", dir_enum = direction.enum_name());

	gen_self_impl_block(Trait::State, &path, dir_lifetime)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Trait {
	Version,
	Packet,
	State,
}

impl Trait {
	fn trait_name(self) -> &'static str {
		match self {
			Trait::Version => "IntoVersionEnum",
			Trait::Packet => "IntoPacketEnum",
			Trait::State => "IntoStateEnum",
		}
	}
	fn assoc_type(self) -> &'static str {
		match self {
			Trait::Version => "Packet",
			Trait::Packet => "State",
			Trait::State => "Direction",
		}
	}
	fn method_name(self) -> &'static str {
		match self {
			Trait::Version => "into_version_enum",
			Trait::Packet => "into_packet_enum",
			Trait::State => "into_state_enum",
		}
	}
}

fn gen_impl_block(
	// trait to be implemented
	t: Trait,
	// path of the item for which to implement
	path: &str,
	// whether the item has a lifetime
	lifetime: bool,
	// path to the superior enum
	super_path: &str,
	// whether the superior enum has a lifetime
	super_lifetime: bool,
	// which variant of the superior enum it is
	variant: &str,
	// whether to put value as self or the lower enum
	direct: bool,
) -> String {
	let trait_name = t.trait_name();
	let assoc_type = t.assoc_type();
	let method_name = t.method_name();

	let lifetime = get_lifetime(lifetime);
	let super_lifetime = get_lifetime(super_lifetime);

	let value = if direct {
		"self"
	} else {
		match t {
			Trait::Version => "self",
			Trait::Packet => "crate::IntoVersionEnum::into_version_enum(self)",
			Trait::State => "crate::IntoPacketEnum::into_packet_enum(self)",
		}
	};

	format!(
		r#"
        impl<'a> crate::{trait_name}<'a> for {path} {lifetime} {{
            type {assoc_type} = {super_path} {super_lifetime};

            fn {method_name}(self) -> Self::{assoc_type} {{
                let v = {value};
                {super_path}::{variant}(v)
            }}
        }}
       "#
	)
}

fn gen_self_impl_block(
	// trait to be implemented
	t: Trait,
	// path of the item for which to implement (WITH lifetime)
	path: &str,
	// whether this item has a lifetime
	lifetime: bool,
) -> String {
	let trait_name = t.trait_name();
	let assoc_type = t.assoc_type();
	let method_name = t.method_name();

	let lifetime = get_lifetime(lifetime);

	format!(
		r#"
        impl<'a> crate::{trait_name}<'a> for {path} {lifetime} {{
            type {assoc_type} = Self;

            fn {method_name}(self) -> Self::{assoc_type} {{
                self
            }}
        }}
       "#
	)
}
