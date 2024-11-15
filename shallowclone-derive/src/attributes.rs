use crate::DeriveType;
use proc_macro_error::{abort, emit_error};
use syn::{GenericParam, Ident};

pub fn is_generic_skipped(derive_type: DeriveType, input: &GenericParam) -> bool {
	let attrs = match input {
		GenericParam::Lifetime(lifetime_param) => &lifetime_param.attrs,
		GenericParam::Type(type_param) => &type_param.attrs,
		GenericParam::Const(_) => abort!(input, "const generics cannot be skipped"),
	};

	for attr in attrs {
		let root_tag = match derive_type {
			DeriveType::ShallowClone => "shallowclone",
			DeriveType::MakeOwned => "makeowned",
		};
		if !attr.path().is_ident(root_tag) {
			continue;
		}

		if let syn::Meta::List(list) = &attr.meta {
			if let Ok(parsed) = list.parse_args::<Ident>() {
				if parsed.to_string() == "skip" {
					return true;
				} else {
					emit_error!(parsed, "Unknown attribute");
					continue;
				}
			}
		}

		emit_error!(attr, "Unknown attribute");
	}

	false
}
