use crate::{
	internal::{
		read::{read_tag, read_value},
		write::{write_str, write_tag, write_value},
		InternalNbtRead,
	},
	tag::Tag,
	Nbt,
};
use indexmap::IndexMap;
use std::ops::Deref;

pub type NbtCompound = IndexMap<String, NbtValue>;

#[derive(Debug, Clone, PartialEq)]
pub enum NbtValue {
	Byte(i8),
	Short(i16),
	Int(i32),
	Long(i64),
	Float(f32),
	Double(f64),
	ByteArray(NbtByteArray),
	String(String),
	List(NbtList),
	Compound(NbtCompound),
	IntArray(NbtIntArray),
	LongArray(NbtLongArray),
}

impl NbtValue {
	pub(crate) fn tag(&self) -> Tag {
		match self {
			Self::Byte(_) => Tag::Byte,
			Self::Short(_) => Tag::Short,
			Self::Int(_) => Tag::Int,
			Self::Long(_) => Tag::Long,
			Self::Float(_) => Tag::Float,
			Self::Double(_) => Tag::Double,
			Self::ByteArray(_) => Tag::ByteArray,
			Self::String(_) => Tag::String,
			Self::List(_) => Tag::List,
			Self::Compound(_) => Tag::Compound,
			Self::IntArray(_) => Tag::IntArray,
			Self::LongArray(_) => Tag::LongArray,
		}
	}
}

impl Nbt for NbtValue {
	fn nbt_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(self.tag(), output);
		written += write_value(self, output);

		written
	}
	fn nbt_write_named(&self, name: &str, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(self.tag(), output);
		written += write_str(name, output);
		written += write_value(self, output);

		written
	}
	fn nbt_read(input: &mut &[u8]) -> crate::Result<Self> {
		let tag = read_tag(input)?;

		read_value(input, tag)
	}
	fn nbt_read_named(input: &mut &[u8]) -> crate::Result<(String, Self)> {
		let tag = read_tag(input)?;
		let name = String::nbt_iread(input)?;

		read_value(input, tag).map(|v| (name, v))
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum NbtList {
	Byte(Vec<i8>),
	Short(Vec<i16>),
	Int(Vec<i32>),
	Long(Vec<i64>),
	Float(Vec<f32>),
	Double(Vec<f64>),
	ByteArray(Vec<NbtByteArray>),
	String(Vec<String>),
	List(Vec<NbtList>),
	Compound(Vec<NbtCompound>),
	IntArray(Vec<NbtIntArray>),
	LongArray(Vec<NbtLongArray>),
}

impl NbtList {
	pub(crate) fn tag(&self) -> Tag {
		match self {
			Self::Byte(_) => Tag::Byte,
			Self::Short(_) => Tag::Short,
			Self::Int(_) => Tag::Int,
			Self::Long(_) => Tag::Long,
			Self::Float(_) => Tag::Float,
			Self::Double(_) => Tag::Double,
			Self::ByteArray(_) => Tag::ByteArray,
			Self::String(_) => Tag::String,
			Self::List(_) => Tag::List,
			Self::Compound(_) => Tag::Compound,
			Self::IntArray(_) => Tag::IntArray,
			Self::LongArray(_) => Tag::LongArray,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct NbtByteArray(pub Vec<i8>);

#[derive(Debug, Clone, PartialEq)]
pub struct NbtIntArray(pub Vec<i32>);

#[derive(Debug, Clone, PartialEq)]
pub struct NbtLongArray(pub Vec<i64>);

impl Deref for NbtByteArray {
	type Target = Vec<i8>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl Deref for NbtIntArray {
	type Target = Vec<i32>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl Deref for NbtLongArray {
	type Target = Vec<i64>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
