use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, GenericParam, parse_macro_input};

/// Derives the `Nbt` traits for your struct.
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
			type StaticSize = ::craftflow_nbt::internal::U0;
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
	let field_types: Vec<_> = fields.named.iter().map(|f| f.ty.clone()).collect();

	quote! {
		#(
			let mut #field_vars = None;
		)*

		loop {
			let ___tag = ::craftflow_nbt::internal::read::read_tag(___input)?;
			if ___tag == ::craftflow_nbt::Tag::End {
				break;
			}

			let ___key = String::nbt_iread(___input)?;

			#(
				if #field_names_str == ___key {
					if #field_vars.is_some() {
						return Err(::craftflow_nbt::Error::KeyCollision(___key));
					}
					if <#field_types as ::craftflow_nbt::internal::InternalNbt>::TAG != ___tag {
						return Err(::craftflow_nbt::Error::WrongTag{
							field_name: #field_names_str,
							expected: <#field_types as ::craftflow_nbt::internal::InternalNbt>::TAG,
							found: ___tag,
						});
					}

					let ___value = <#field_types as ::craftflow_nbt::internal::InternalNbtRead>::nbt_iread(___input)?;
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
			if #field_vars.is_none() {
				___not_found_fields.push(#field_names_str);
			}
		)*

		if !___not_found_fields.is_empty() {
			// at least one field was not parsed
			return Err(::craftflow_nbt::Error::MissingKeys(___not_found_fields));
		}

		Ok(Self {
			#(
				#field_vars: #field_vars.unwrap(),
			)*
		})
	}
}

fn gen_write_impl(fields: &FieldsNamed) -> TokenStream2 {
	let field_vars: Vec<_> = fields
		.named
		.iter()
		.map(|f| f.ident.clone().unwrap())
		.collect();
	let field_names_str: Vec<_> = field_vars.iter().map(|f| format!("{f}")).collect();
	let field_types: Vec<_> = fields.named.iter().map(|f| f.ty.clone()).collect();

	quote! {
		let mut ___written = 0usize;

		#(
			___written += ::craftflow_nbt::internal::write::write_tag(<#field_types as ::craftflow_nbt::internal::InternalNbt>::TAG, ___output);
			___written += ::craftflow_nbt::internal::write::write_str(#field_names_str, ___output);
			___written += self.#field_vars.nbt_iwrite(___output);
		)*

		___written += ::craftflow_nbt::internal::write::write_tag(::craftflow_nbt::Tag::End, ___output);

		___written
	}
}
