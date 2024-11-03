use crate::parse_packet_info::{Directions, Generics, PacketType};

/// Generates a macro that implements a given trait for all
pub fn gen_impl_trait_macro(directions: &Directions) -> String {
	let mut inner = String::new();
	let mut inner_post = String::new();

	for (direction, (_, states)) in directions {
		let dir_mod = direction.mod_name();

		for (state, (_, packets)) in states {
			let st_mod = state.mod_name();

			for (packet, (_, versions)) in packets {
				let pkt_mod = packet.mod_name();

				for (version, info) in versions {
					// skip re-exports
					let (type_name, generics) = match &info.packet_type {
						PacketType::ReExport { .. } => continue,
						PacketType::Defined {
							type_name,
							generics,
						} => (type_name, generics),
					};
					let v_mod = version.mod_name();

					let path = format!(
                        "::craftflow_protocol_versions::{dir_mod}::{st_mod}::{pkt_mod}::{v_mod}::{type_name}"
                    );
					inner += &gen_impl(&path, generics, false);
					inner_post += &gen_impl(&path, generics, true);
				}
			}
		}
	}

	format!(
		"// This macro is used internally in craftflow for the packet events.
		#[doc(hidden)]
		#[macro_export]
		macro_rules! __gen_impls_for_packets__ {{
		    (impl $trait:ident for X $code:tt) => {{ {inner} }};
			// Instead of making this slop 100x more complicated, we just handle the specific Post newtype that we need
		    (impl $trait:ident for Post<X> $code:tt) => {{ {inner_post} }};
		}}"
	)
}

fn gen_impl(path: &str, generics: &Generics, post: bool) -> String {
	let generics = generics.as_str();
	let target = if post {
		format!("Post<X {generics}>")
	} else {
		format!("X {generics}")
	};

	format!(
		r#"
        const _: () = {{
			type X {generics} = {path} {generics};
			impl {generics} $trait for {target} $code
		}};
	"#
	)
}
