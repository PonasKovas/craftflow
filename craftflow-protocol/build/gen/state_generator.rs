use super::{
	enum_generator::EnumGenerator, feature::Feature, packet_generator::PacketGenerator,
	struct_generator::StructGenerator,
};
use crate::build::{
	gen::feature::FeatureCfgOptions,
	info_file::Info,
	util::{to_pascal_case, StateName},
};
use proc_macro2::TokenStream;
use quote::quote;

pub struct StateGenerator {
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

		quote! {
			#feature_cfg
			use #module_name::*;

			#feature_cfg
			#[doc = #main_enum_comment]
			#main_enum

			#feature_cfg
			#[doc = #module_comment]
			pub mod #module_name {
				#[allow(unused_imports)]
				use crate::datatypes::*;

				#( #packets )*
				#( #structs )*
				#( #enums )*
			}
		}
	}
}
