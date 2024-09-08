use proc_macro2::{Ident, TokenStream};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
	C2S,
	S2C,
}

#[derive(Debug, PartialEq, Clone, Eq, Ord, PartialOrd)]
pub struct StateName {
	pub name: String,
}

impl Direction {
	pub fn module(&self) -> Ident {
		match self {
			Direction::C2S => "c2s".as_ident(),
			Direction::S2C => "s2c".as_ident(),
		}
	}
	pub fn enum_name(&self) -> Ident {
		match self {
			Direction::C2S => "C2S".as_ident(),
			Direction::S2C => "S2C".as_ident(),
		}
	}
}

impl StateName {
	pub fn module(&self) -> Ident {
		self.name.as_ident()
	}
	pub fn enum_name(&self) -> Ident {
		format!("{}Packet", to_pascal_case(&self.name)).as_ident()
	}
	pub fn direction_enum_variant(&self) -> Ident {
		to_pascal_case(&self.name).as_ident()
	}
}

pub trait AsIdent {
	fn as_ident(&self) -> Ident;
}
impl AsIdent for str {
	fn as_ident(&self) -> Ident {
		Ident::new(self, proc_macro2::Span::call_site())
	}
}

pub trait AsTokenStream {
	fn as_tokenstream(&self) -> TokenStream;
}
impl AsTokenStream for str {
	fn as_tokenstream(&self) -> TokenStream {
		TokenStream::from_str(self).unwrap()
	}
}

pub fn to_pascal_case(s: &str) -> String {
	fn capitalize(s: &str) -> String {
		let mut c = s.chars();
		match c.next() {
			None => String::new(),
			Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
		}
	}

	s.split('_').map(|word| capitalize(word)).collect()
}
