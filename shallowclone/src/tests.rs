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
#[shallowclone(target = "StructGeneric<T::Target>")]
struct StructGeneric<#[shallowclone] T> {
	field: Option<T>,
}

#[derive(ShallowClone)]
#[shallowclone(target = "Enum<'shallowclone, T::Target>")]
enum Enum<'a, #[shallowclone] T> {
	UnitVariant,
	TupleVariant(u16, u32, String),
	StructVariant {
		field1: &'a u16,
		field2: T,
		field3: Cow<'a, str>,
	},
}

#[derive(ShallowClone, Clone)]
#[shallowclone(target = "Complex<'shallowclone, 'b>")]
struct Complex<'a, 'b> {
	field: Cow<'a, [Complex<'b, 'b>]>,
}

#[derive(ShallowClone)]
#[shallowclone(target = "Array<'shallowclone, T>")]
pub struct Array<'a, T: Clone> {
	pub data: Cow<'a, [T]>,
}
