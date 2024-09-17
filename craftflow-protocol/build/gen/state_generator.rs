use super::{
	enum_generator::EnumGenerator, feature::Feature, packet_generator::PacketGenerator,
	struct_generator::StructGenerator,
};
use crate::build::{
	gen::feature::FeatureCfgOptions,
	info_file::Info,
	util::{to_pascal_case, AsIdent, Direction, StateName},
};
use proc_macro2::TokenStream;
use quote::quote;

pub struct StateGenerator {
	pub direction: Direction,
	pub name: StateName,
	pub feature: Option<Feature>,
	/// The enum that contains all packets of this state
	pub main_enum: EnumGenerator,
	pub packets: Vec<PacketGenerator>,
	pub structs: Vec<StructGenerator>,
	pub enums: Vec<EnumGenerator>,
}

impl StateGenerator {
	/// Generates an enum with a variant for each possible packet and
	/// a module containing all packets, structs and enums of this state
	pub fn gen(&self, info: &Info) -> TokenStream {
		let feature_cfg = self.feature.as_ref().map(|f| {
			f.gen_cfg(FeatureCfgOptions {
				negative: false,
				with_doc_note: true,
			})
		});
		let module_name = self.name.module();

		let main_enum = self.main_enum.gen(info);

		let packets = self.packets.iter().fold(Vec::new(), |mut v, p| {
			v.push(p.gen(info));
			v
		});
		let structs = self.structs.iter().fold(Vec::new(), |mut v, s| {
			v.push(s.gen(info));
			v
		});
		let enums = self.enums.iter().fold(Vec::new(), |mut v, e| {
			v.push(e.gen(info));
			v
		});

		let main_enum_comment = format!(
			"Enum containing all possible packets of the `{}` state.",
			to_pascal_case(&self.name.name)
		);
		let module_comment = format!(
			"Module containing all packets, structs and enums of the `{}` state.",
			to_pascal_case(&self.name.name)
		);

		let destructure_macro = gen_destructure_macro(self);

		let direction_enum = self.direction.enum_name();
		let main_enum_name = self.name.enum_name();
		let direction_enum_variant = self.name.direction_enum_variant();

		quote! {
			#feature_cfg
			#[doc = #main_enum_comment]
			#main_enum

			#feature_cfg
			#[doc = #module_comment]
			pub mod #module_name {
				#[allow(unused_imports)]
				use crate::datatypes::*;
				#[allow(unused_imports)]
				use crate::serde_types;

				#( #packets )*
				#( #structs )*
				#( #enums )*

				#destructure_macro
			}

			impl Into<crate::protocol::#direction_enum> for #main_enum_name {
				fn into(self) -> crate::protocol::#direction_enum {
					crate::protocol::#direction_enum::#direction_enum_variant(self)
				}
			}
		}
	}
}

fn gen_destructure_macro(state: &StateGenerator) -> TokenStream {
	let macro_name = format!(
		"destructure_packet_{}_{}",
		state.direction.module(),
		state.name.name
	)
	.as_ident();

	let direction = state.direction.module();
	let main_enum = state.name.enum_name();

	let mut arms = Vec::new();
	for variant in &state.main_enum.variants {
		let feature_cfg = variant.feature.as_ref().map(|f| {
			f.gen_cfg(FeatureCfgOptions {
				negative: false,
				with_doc_note: false,
			})
		});
		let variant_name = &variant.name;
		let packet_type = &variant.fields.fields[0].datatype;

		arms.push(quote! {
			#feature_cfg
			$crate::protocol::#direction::#main_enum::#variant_name{ packet } => {
				Some($fn::<$crate::protocol::#direction::#packet_type>(packet, ($($args,)*)))
			},
		});
	}

	let doc = format!(
		r#"A macro that expands to a `match` statement matching every packet and then doing something with the destructured
packet (inner packet type, without the enum)

Usage:
```
# use craftflow_protocol::{macro_name};
# let state_packet_enum = craftflow_protocol::protocol::{direction}::{main_enum}::_Unsupported;

fn my_function<P>(packet: P, args:(u32,)) -> bool {{
	args.0 == 0
}}

let x = 123_u32;

// Returns None if the packet was _Unsupported
let result: Option<bool> = {macro_name}!(
	state_packet_enum,
	my_function,
 	(x),
);
```
"#
	);

	quote! {
		#[doc = #doc]
		#[macro_export]
		macro_rules! #macro_name {
			($packet:ident, $fn:ident, ($($args:expr),*) $(,)?) => {
				match $packet {
					#( #arms )*
					$crate::protocol::#direction::#main_enum::_Unsupported => None,
				}
			}
		}
	}
}
