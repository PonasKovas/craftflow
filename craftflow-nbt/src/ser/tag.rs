use crate::{
	arrays::{MAGIC_BYTE_ARRAY, MAGIC_INT_ARRAY, MAGIC_LONG_ARRAY},
	tag::Tag,
	Error,
};
use serde::{
	ser::{
		SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
		SerializeTupleStruct, SerializeTupleVariant,
	},
	Serializer,
};

/// Just returns the tag of the given data
/// This is used for the serialization of lists, where we must save the tag of the first
/// element to make sure all subsequent elements have the same tag
pub struct TagSerializer;

impl Serializer for TagSerializer {
	type Ok = Tag;
	type Error = Error;

	type SerializeSeq = Self;
	type SerializeTuple = Self;
	type SerializeTupleStruct = Self;
	type SerializeTupleVariant = Self;
	type SerializeMap = Self;
	type SerializeStruct = Self;
	type SerializeStructVariant = Self;

	fn serialize_newtype_struct<T>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		let tag = match name {
			MAGIC_BYTE_ARRAY => Tag::ByteArray,
			MAGIC_INT_ARRAY => Tag::IntArray,
			MAGIC_LONG_ARRAY => Tag::LongArray,
			_ => value.serialize(self)?,
		};

		Ok(tag)
	}

	fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Byte)
	}
	fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Byte)
	}
	fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Short)
	}
	fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Int)
	}
	fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Long)
	}
	fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Byte)
	}
	fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Short)
	}
	fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Int)
	}
	fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Long)
	}
	fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Float)
	}
	fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Double)
	}
	fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("char is not supported")))
	}
	fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::String)
	}
	fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::List)
	}
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::End)
	}
	fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		value.serialize(self)
	}
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::End)
	}
	fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::End)
	}
	fn serialize_unit_variant(
		self,
		_: &'static str,
		_: u32,
		_: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::End)
	}
	fn serialize_newtype_variant<T>(
		self,
		_: &'static str,
		_: u32,
		_: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		value.serialize(self)
	}
	fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Ok(self)
	}
	fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Ok(self)
	}
	fn serialize_tuple_struct(
		self,
		_: &'static str,
		_: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Ok(self)
	}
	fn serialize_tuple_variant(
		self,
		_: &'static str,
		_: u32,
		_: &'static str,
		_: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Ok(self)
	}
	fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Ok(self)
	}
	fn serialize_struct(
		self,
		_: &'static str,
		_: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		Ok(self)
	}
	fn serialize_struct_variant(
		self,
		_: &'static str,
		_: u32,
		_: &'static str,
		_: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		Ok(self)
	}
}
impl SerializeSeq for TagSerializer {
	type Ok = Tag;
	type Error = Error;

	fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::List)
	}
}
impl SerializeTuple for TagSerializer {
	type Ok = Tag;
	type Error = Error;

	fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::List)
	}
}
impl SerializeTupleStruct for TagSerializer {
	type Ok = Tag;
	type Error = Error;

	fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::List)
	}
}
impl SerializeTupleVariant for TagSerializer {
	type Ok = Tag;
	type Error = Error;

	fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::List)
	}
}
impl SerializeMap for TagSerializer {
	type Ok = Tag;
	type Error = Error;

	fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Compound)
	}
}
impl SerializeStruct for TagSerializer {
	type Ok = Tag;
	type Error = Error;

	fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Compound)
	}
}
impl SerializeStructVariant for TagSerializer {
	type Ok = Tag;
	type Error = Error;

	fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Tag::Compound)
	}
}
