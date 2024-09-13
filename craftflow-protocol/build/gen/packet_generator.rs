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
	/// Generates a struct definition, MinecraftProtocol implementation and Packet implementation
	pub fn gen(&self, info: &Info) -> TokenStream {
		let mut result = self.inner.gen(info);

		let struct_name = &self.inner.name;
		let struct_generics = &self.inner.generics.gen();

		let direction_enum_generics = if self.inner.generics.has_a_lifetime() {
			quote! { <'a> }
		} else {
			quote! { <'static> }
		};

		let static_self_generics = self.inner.generics.gen_with_static_lifetime();

		let direction_enum_name = self.direction.enum_name();
		let direction_enum_variant = self.state_name.direction_enum_variant();

		let state_enum_name = self.state_name.enum_name();

		result.extend(quote! {
			impl #struct_generics crate::Packet for #struct_name #struct_generics {
				type Direction = crate::protocol::#direction_enum_name #direction_enum_generics;
				type StaticSelf = #struct_name #static_self_generics;

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
