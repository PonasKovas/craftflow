use crate::ShallowClone;
use std::borrow::Cow;

#[derive(ShallowClone)]
struct UnitStruct;

#[derive(ShallowClone)]
struct EmptyStruct {}

#[derive(ShallowClone)]
struct TupleStruct(u16, u32, String);

#[derive(ShallowClone)]
struct Struct {
	field1: u16,
	field2: u32,
	field3: String,
}

#[derive(ShallowClone)]
struct StructGeneric<T> {
	field: Option<T>,
}

#[derive(ShallowClone)]
enum Enum<'a, T> {
	UnitVariant,
	TupleVariant(u16, u32, String),
	StructVariant {
		field1: &'a u16,
		field2: T,
		field3: Cow<'a, str>,
	},
}

#[derive(ShallowClone)]
struct Array<'a, #[shallowclone(skip)] T: Clone> {
	pub data: Cow<'a, [T]>,
}

#[derive(ShallowClone, Clone)]
struct Complex<'a> {
	field: ComplexCow<'a>,
}

#[derive(ShallowClone, Clone)]
enum ComplexCow<'a> {
	Owned(Vec<Complex<'a>>),
	Borrowed(&'a [Complex<'a>]),
}
