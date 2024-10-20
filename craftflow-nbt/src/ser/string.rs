use super::any::AnySerializer;
use crate::{tag::Tag, Error};
use serde::{ser::Impossible, Serialize, Serializer};
use std::io::Write;

/// A serializer that only accepts a string
/// This is used to serialize keys of compounds
pub struct StrSerializer<W> {
	pub output: W,
}

impl<'a, W: Write> Serializer for StrSerializer<W> {
	type Ok = usize;
	type Error = Error;

	type SerializeSeq = Impossible<usize, Error>;
	type SerializeTuple = Impossible<usize, Error>;
	type SerializeTupleStruct = Impossible<usize, Error>;
	type SerializeTupleVariant = Impossible<usize, Error>;
	type SerializeMap = Impossible<usize, Error>;
	type SerializeStruct = Impossible<usize, Error>;
	type SerializeStructVariant = Impossible<usize, Error>;

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		let serializer = AnySerializer {
			output: self.output,
			expecting: Some(Tag::String),
		};

		serializer.serialize_str(v)
	}
	fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(name)
	}
	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(variant)
	}

	fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		value.serialize(self)
	}
	fn serialize_newtype_struct<T>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		value.serialize(self)
	}
	fn serialize_newtype_variant<T>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		value.serialize(self)
	}

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got bool {v}")))
	}
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got i8 {v}")))
	}
	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got i16 {v}")))
	}
	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got i32 {v}")))
	}
	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got i64 {v}")))
	}
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got u8 {v}")))
	}
	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got u16 {v}")))
	}
	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got u32 {v}")))
	}
	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got u64 {v}")))
	}
	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got f32 {v}")))
	}
	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got f64 {v}")))
	}
	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got char {v}")))
	}
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got bytes {v:?}")))
	}
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got none")))
	}
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got unit")))
	}
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got seq")))
	}
	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got tuple")))
	}
	fn serialize_tuple_struct(
		self,
		name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Err(Error::InvalidData(format!(
			"expected str, got tuple struct {name:?}"
		)))
	}
	fn serialize_tuple_variant(
		self,
		name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Err(Error::InvalidData(format!(
			"expected str, got tuple variant {name:?} {variant:?}"
		)))
	}
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Err(Error::InvalidData(format!("expected str, got map")))
	}
	fn serialize_struct(
		self,
		name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		Err(Error::InvalidData(format!(
			"expected str, got struct {name:?}"
		)))
	}
	fn serialize_struct_variant(
		self,
		name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		Err(Error::InvalidData(format!(
			"expected str, got struct variant {name:?} {variant:?}"
		)))
	}
}
