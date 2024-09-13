use crate::build::util::AsTokenStream;
use proc_macro2::TokenStream;
use quote::quote;

/// <'a, 'b, const C: usize, D, E>
pub struct Generics {
	pub generics: Vec<String>,
}

impl Generics {
	pub fn parse(input: &str) -> Self {
		let mut s = match input.find('<') {
			Some(start) => &input[(start + 1)..],
			None => {
				return Generics {
					generics: Vec::new(),
				};
			}
		};

		s = s.strip_suffix('>').expect("type has < but not >");

		let mut result = Vec::new();
		for element in s.split(',') {
			result.push(element.trim().to_owned());
		}

		Self { generics: result }
	}
	// Generates the <A, B, C> generics adding an 'a lifetime if its not already there
	pub fn gen_with_a_lifetime(&self) -> TokenStream {
		let a_lifetime = if !self.has_a_lifetime() {
			Some(quote! { 'a })
		} else {
			None
		}
		.into_iter();

		let generics = self.generics.iter().map(|g| g.as_tokenstream());

		quote! { < #( #a_lifetime, )* #( #generics, )* > }
	}
	// Generates the <A, B, C> generics replacing the 'a lifetime with 'static
	pub fn gen_with_static_lifetime(&self) -> TokenStream {
		let generics = self
			.generics
			.iter()
			.map(|g| if g == "'a" { "'static" } else { g }.as_tokenstream());

		quote! { < #( #generics, )* > }
	}
	// Generates the <A, B, C> generics
	pub fn gen(&self) -> TokenStream {
		let generics = self.generics.iter().map(|g| g.as_tokenstream());

		quote! { < #( #generics, )* > }
	}
	// check if 'a lifetime generic is there
	pub fn has_a_lifetime(&self) -> bool {
		self.generics.iter().find(|g| *g == "'a").is_some()
	}
}
