use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, GenericParam, LitStr, parse_macro_input};

/// Creates a static `NbtStr`
///
/// ```
/// const S: &'static NbtStr = nbtstr!("hello");
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn nbtstr(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as LitStr);

	fn calc_mcesu8_len(s: &str) -> usize {
		let mut extra = 0;
		for c in s.chars() {
			if c == '\u{0}' {
				extra += 1; // NUL is represented as \xC0 \x80
			}
			if c > '\u{FFFF}' {
				extra += 2; // each 4-byte UTF-8 sequence (BMP > U+FFFF) becomes 6 bytes in CESU-8 (2 extra bytes per character).
			}
		}

		s.len() + extra
	}

	if calc_mcesu8_len(&input.value()) > u16::MAX as usize {
		abort_call_site!("Length exceeded! Max length {}", u16::MAX);
	}

	quote! {
		unsafe {
			::craftflow_nbt::NbtStr::new_unchecked(#input)
		}
	}
	.into()
}

/// Derives the `NbtRead` and `NbtWrite` traits for your struct.
///
/// Must have named fields, as the names will be used as keys of the Nbt compound.
#[proc_macro_error]
#[proc_macro_derive(Nbt)]
pub fn derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let data = match input.data {
		Data::Struct(s) => s,
		_ => abort_call_site!("Nbt can only be derived for structs"),
	};
	let fields = match data.fields {
		Fields::Named(f) => f,
		_ => abort_call_site!("Nbt can only be derived for structs with named fields"),
	};

	let ident = &input.ident;
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

	let mut extra_bounds = Vec::new();
	for generic in &input.generics.params {
		if let GenericParam::Type(type_param) = generic {
			let name = &type_param.ident;
			extra_bounds.push(quote! { #name: ::craftflow_nbt::internal::InternalNbtRead + ::craftflow_nbt::internal::InternalNbtWrite });
		}
	}

	// i fucking hate syn
	let where_clause = where_clause
		.map(|c| quote! { #c })
		.unwrap_or(quote! { where });
	let where_clause = quote! {
		#where_clause
		#(#extra_bounds),*
	};

	let read_impl = gen_read_impl(&fields);
	let write_impl = gen_write_impl(&fields);

	quote! {
		impl #impl_generics ::craftflow_nbt::internal::InternalNbt for #ident #type_generics
		#where_clause {
			const TAG: ::craftflow_nbt::Tag = ::craftflow_nbt::Tag::Compound;
			const IS_POD: bool = false;
		}

		impl #impl_generics ::craftflow_nbt::internal::InternalNbtRead for #ident #type_generics
		#where_clause {
			fn nbt_iread(___input: &mut &[u8]) -> ::craftflow_nbt::Result<Self> {
				#read_impl
			}
		}

		impl #impl_generics ::craftflow_nbt::internal::InternalNbtWrite for #ident #type_generics
		#where_clause {
			fn nbt_iwrite(&self, ___output: &mut Vec<u8>) -> usize {
				#write_impl
			}
		}
	}
	.into()
}

fn gen_read_impl(fields: &FieldsNamed) -> TokenStream2 {
	let field_vars: Vec<_> = fields
		.named
		.iter()
		.map(|f| f.ident.clone().unwrap())
		.collect();
	let field_names_str: Vec<_> = field_vars.iter().map(|f| format!("{f}")).collect();

	let mut field_types = Vec::new();
	let mut field_is_required = Vec::new();
	let mut field_unwrap = Vec::new();
	for field in &fields.named {
		let option = get_option_inner(&field.ty);

		field_types.push(option.unwrap_or(&field.ty).clone());
		field_is_required.push(if option.is_none() {
			quote! { true }
		} else {
			quote! { false }
		});
		field_unwrap.push(option.is_none().then(|| quote! { .unwrap() }));
	}

	quote! {
		#(
			let mut #field_vars = None;
		)*

		loop {
			let ___tag = ::craftflow_nbt::internal::read::read_tag(___input)?;
			if ___tag == ::craftflow_nbt::Tag::End {
				break;
			}

			let ___key = ::craftflow_nbt::NbtString::nbt_iread(___input)?;

			#(
				if #field_names_str == ___key {
					if #field_vars.is_some() {
						return Err(::craftflow_nbt::Error::KeyCollision(___key));
					}
					if <#field_types as ::craftflow_nbt::internal::InternalNbt>::TAG != ___tag {
						return Err(::craftflow_nbt::Error::WrongTag{
							field_name: #field_names_str.into(),
							expected: <#field_types as ::craftflow_nbt::internal::InternalNbt>::TAG,
							found: ___tag,
						});
					}

					let ___value = match <#field_types as ::craftflow_nbt::internal::InternalNbtRead>::nbt_iread(___input) {
						Ok(___value) => ___value,
						// Sequences may encounter a wrong type but they don't know which field they are in, so we catch those
						// and add this info for better errors
						Err(::craftflow_nbt::Error::WrongSeqTag{ field_name: None, expected: ___expected, found: ___found }) => {
							return Err(::craftflow_nbt::Error::WrongSeqTag {
								field_name: Some(#field_names_str),
								expected: ___expected,
								found: ___found,
							})
						},
						Err(___other) => return Err(___other),
					};
					#field_vars = Some(___value);
					continue;
				}
			)*

			// if made it here, that means the key didnt match any of the fields
			// so just parse whatever it may be dynamically and discard
			::craftflow_nbt::internal::read::read_value(___input, ___tag)?;
		}

		// make sure all fields are found
		let mut ___not_found_fields = Vec::new();

		#(
			if #field_is_required && #field_vars.is_none() {
				___not_found_fields.push(#field_names_str);
			}
		)*

		if !___not_found_fields.is_empty() {
			// at least one field was not parsed
			return Err(::craftflow_nbt::Error::MissingKeys(___not_found_fields));
		}

		Ok(Self {
			#(
				#field_vars: #field_vars #field_unwrap,
			)*
		})
	}
}

fn gen_write_impl(fields: &FieldsNamed) -> TokenStream2 {
	let mut field_write = Vec::new();
	for field in &fields.named {
		let name = field.ident.as_ref().unwrap();
		let name_str = format!("{name}");

		let option = get_option_inner(&field.ty);
		let field_type = option.unwrap_or(&field.ty);

		let inner = quote! {
			___written += ::craftflow_nbt::internal::write::write_tag(<#field_type as ::craftflow_nbt::internal::InternalNbt>::TAG, ___output);
			___written += ::craftflow_nbt::nbtstr!(#name_str).nbt_iwrite(___output);
			___written += #name.nbt_iwrite(___output);
		};

		field_write.push(if option.is_some() {
			quote! {
				if let Some(#name) = &self.#name {
					#inner
				}
			}
		} else {
			quote! {
				let #name = &self.#name;
				#inner
			}
		});
	}

	quote! {
		let mut ___written = 0usize;

		#( #field_write )*

		___written += ::craftflow_nbt::internal::write::write_tag(::craftflow_nbt::Tag::End, ___output);

		___written
	}
}

/// Checks if a type is Option<...>, and if so, returns the inner type
fn get_option_inner(ty: &syn::Type) -> Option<&syn::Type> {
	if let syn::Type::Path(type_path) = ty {
		if let Some(segment) = type_path.path.segments.last() {
			if segment.ident == "Option" {
				if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
					if args.args.len() == 1 {
						if let syn::GenericArgument::Type(inner) = &args.args[0] {
							return Some(inner);
						}
					}
				}
			}
		}
	}
	None
}
