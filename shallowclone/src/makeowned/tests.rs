use crate::MakeOwned;
use std::{borrow::Cow, marker::PhantomData};

#[derive(MakeOwned, Clone)]
struct UnitStruct;

#[derive(MakeOwned, Clone)]
struct EmptyStruct {}

#[derive(MakeOwned, Clone)]
struct TupleStruct(u16, u32, String);

#[derive(MakeOwned, Clone)]
struct Struct {
	field1: u16,
	field2: u32,
	field3: String,
}

#[derive(MakeOwned, Clone)]
struct StructGeneric<T> {
	field: Option<T>,
}

#[derive(MakeOwned, Clone)]
enum Enum<'a, T> {
	UnitVariant,
	TupleVariant(u16, u32, String),
	StructVariant {
		field1: [u16; 16],
		field2: T,
		field3: Cow<'a, str>,
	},
}

#[derive(MakeOwned, Clone)]
struct Array<'a, T: Clone> {
	data: Cow<'a, [T]>,
}

#[derive(MakeOwned, Clone)]
struct WithPhantom<#[makeowned(skip)] T> {
	inner: PhantomData<T>,
}

#[derive(MakeOwned, Clone)]
pub struct HoverActionShowEntity<'a> {
	/// The textual identifier of the entity's type. If unrecognized, defaults to minecraft:pig.
	pub entity_type: Cow<'a, str>,
	/// The entity's UUID (with dashes). Does not need to correspond to an existing entity; only for display.
	pub id: Cow<'a, str>,
	/// The entity's custom name.
	pub name: Option<Cow<'a, str>>,
}
