use super::struct_generator::StructGenerator;
use crate::build::{
	info_file::Info,
	util::{Direction, StateName},
};
use proc_macro2::TokenStream;
use quote::quote;

/// This is basically a struct generator that also implements Packet trait
pub struct PacketGenerator {
	pub inner: StructGenerator,
	pub direction: Direction,
	pub state_name: StateName,
}

impl PacketGenerator {
	/// Generates a struct definition, MCPRead and MCPWrite implementations and Packet implementation
	pub fn gen(&self, info: &Info) -> TokenStream {
		let mut result = self.inner.gen(info);

		let struct_name = &self.inner.name;

		let direction_enum_name = self.direction.enum_name();
		let direction_enum_variant = self.state_name.direction_enum_variant();

		let state_enum_name = self.state_name.enum_name();

		result.extend(quote! {
			impl crate::Packet for #struct_name {
				type Direction = crate::protocol::#direction_enum_name;

				fn into_packet_enum(self) -> Self::Direction {
					crate::protocol::#direction_enum_name::#direction_enum_variant(
						super::#state_enum_name::#struct_name{
							packet: self
						}
					)
				}
			}
		});

		result
	}
}
