use super::{
	gen::{
		custom_format::CustomFormat,
		direction_generator::DirectionGenerator,
		enum_generator::{EnumGenerator, Variant},
		feature::Feature,
		field::{Field, FieldFormat},
		fields_container::FieldsContainer,
		packet_generator::PacketGenerator,
		state_generator::StateGenerator,
		struct_generator::StructGenerator,
	},
	state_spec::{SpecItem, StateSpec},
	util::{AsIdent, AsTokenStream, Direction, StateName},
	version_bounds::Bounds,
};
use indexmap::IndexMap;
use quote::quote;
use std::collections::BTreeMap;

// Converts a single direction spec to a direction generator
pub fn spec_to_generator(
	direction: Direction,
	specs: BTreeMap<StateName, StateSpec>,
) -> DirectionGenerator {
	let mut states = Vec::new();

	for (state_name, spec) in specs {
		let mut packets = Vec::new();
		let mut structs = Vec::new();
		let mut enums = Vec::new();
		let mut main_enum_variants = Vec::new();
		for (item_name, item) in spec.items {
			match item {
				SpecItem::Packet(item) => {
					// PACKET
					////////////
					let packet_name = item_name.as_ident();
					let feature = item.feature.map(|feature| Feature { feature });

					main_enum_variants.push(Variant {
						name: packet_name.clone(),
						feature: feature.clone(),
						tags: item
							.id
							.expand_shortcut()
							.into_iter()
							.map(|(bounds, id)| (bounds, quote! { #id }))
							.collect::<IndexMap<_, _>>()
							.into(),
						fields: FieldsContainer {
							fields: vec![Field {
								name: "packet".as_ident(),
								datatype: quote! { #packet_name },
								feature: None,
							}],
							format: IndexMap::from([(
								vec![Bounds::All],
								vec![FieldFormat {
									field: Some("packet".as_ident()),
									format: CustomFormat::default(),
								}],
							)])
							.into(),
						},
					});

					packets.push(PacketGenerator {
						inner: StructGenerator {
							name: packet_name,
							feature,
							fields: FieldsContainer::from_spec(item.data, item.format),
						},
						direction,
						state_name: state_name.clone(),
					});
				}
				SpecItem::Struct(item) => {
					// STRUCT
					////////////
					structs.push(StructGenerator {
						name: item_name.as_ident(),
						feature: item.feature.map(|feature| Feature { feature }),
						fields: FieldsContainer::from_spec(item.data, item.format),
					});
				}
				SpecItem::Enum(item) => {
					// ENUM
					////////////
					let mut variants = Vec::new();

					for (variant_name, variant) in item.variants {
						variants.push(Variant {
							name: variant_name.as_ident(),
							feature: variant.feature.map(|feature| Feature { feature }),
							tags: variant
								.tag
								.expand_shortcut()
								.into_iter()
								.map(|(bounds, id)| (bounds, id.as_tokenstream()))
								.collect::<IndexMap<_, _>>()
								.into(),
							fields: FieldsContainer::from_spec(
								variant.data.unwrap_or(IndexMap::new()),
								variant.format,
							),
						});
					}

					enums.push(EnumGenerator {
						name: item_name.as_ident(),
						feature: item.feature.map(|feature| Feature { feature }),
						variants,
						tag_format: match item.tag_format {
							None => IndexMap::from([(vec![Bounds::All], CustomFormat::default())])
								.into(),
							Some(format) => format
								.expand_shortcut()
								.into_iter()
								.map(|(bounds, format)| {
									(bounds, CustomFormat::from_tag_format(&format))
								})
								.collect::<IndexMap<_, _>>()
								.into(),
						},
					});
				}
			}
		}

		let state_feature = spec.feature.map(|feature| Feature { feature });

		let main_enum = EnumGenerator {
			name: state_name.enum_name(),
			feature: state_feature.clone(),
			variants: main_enum_variants,
			tag_format: IndexMap::from([(
				vec![Bounds::All],
				// This custom format will default to VarInt
				// which is exactly what we need for the packet IDs
				CustomFormat::default(),
			)])
			.into(),
		};

		states.push(StateGenerator {
			direction,
			name: state_name.clone(),
			feature: state_feature,
			main_enum,
			packets,
			structs,
			enums,
		});
	}

	DirectionGenerator { states }
}
