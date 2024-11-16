//! This is not a general purpose macro, it's strictly for internal craftflow implementation.
//! Not intented for any other use. Hidden from the docs.

use crate::parse_packet_info::{Directions, PacketType};

/// Generates a macro which will generate event structs for all packets
pub fn gen_event_macro(directions: &Directions) -> String {
	let mut inner = String::new();

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
						} => (type_name, generics.as_str()),
					};
					let v_mod = version.mod_name();

					let dir = direction.enum_name();
					let state = state.enum_name();
					let packet = packet.enum_name();
					let version = version.caps_mod_name();

					let event_struct = format!("{dir}{state}{packet}{version}Event");
					let packet_path = format!(
                        "::craftflow_protocol_versions::{dir_mod}::{st_mod}::{pkt_mod}::{v_mod}::{type_name} {generics}"
					);

					inner += &format!(
						r#"
						/// Event for the [{dir} {state} {packet} {version}][{packet_path}] packet
						pub struct {event_struct};

						impl $event_trait for {event_struct} {{
						    /// The connection ID and the packet
    						///
    						/// Obviously, don't try to change the connection ID, as it will propagate to other handlers
						    type Args<'a> = (u64, {packet_path});
                            type Return = ();
						}}

						impl<'a> $pointer_trait<'a> for {packet_path} {{
						    type Event = {event_struct};
						}}
					"#
					);
				}
			}
		}
	}

	format!(
		"// This macro is used internally in craftflow for the packet events.
		#[doc(hidden)]
		#[macro_export]
		macro_rules! __gen_events_for_packets__ {{
		    ($event_trait:ident, $pointer_trait:ident) => {{ {inner} }};
		}}"
	)
}
