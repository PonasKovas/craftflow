use crate::MakeOwned;
use std::borrow::Cow;

#[derive(MakeOwned)]
struct UnitStruct;

#[derive(MakeOwned)]
struct EmptyStruct {}

#[derive(MakeOwned)]
struct TupleStruct(u16, u32, String);

#[derive(MakeOwned)]
struct Struct {
	field1: u16,
	field2: u32,
	field3: String,
}

#[derive(MakeOwned)]
struct StructGeneric<T> {
	field: Option<T>,
}

#[derive(MakeOwned)]
enum Enum<'a, T> {
	UnitVariant,
	TupleVariant(u16, u32, String),
	StructVariant {
		field1: [u16; 16],
		field2: T,
		field3: Cow<'a, str>,
	},
}

#[derive(MakeOwned)]
struct Array<'a, T: Clone> {
	pub data: Cow<'a, [T]>,
}
