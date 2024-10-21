use super::write_str::write_str;
use crate::Error;
use serde::{ser::Impossible, Serialize, Serializer};
use std::io::Write;

/// A serializer that only accepts strings
/// This is used to serialize keys of compounds
pub struct CompoundKeySerializer<'a, W> {
	pub output: &'a mut W,
}

fn error<T>() -> Result<T, Error> {
	Err(Error::InvalidData(format!(
		"only strings can be used as a key"
	)))
}

impl<'a, W: Write> Serializer for CompoundKeySerializer<'a, W> {
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
		write_str(self.output, v)
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

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		error()
	}
	fn serialize_newtype_variant<T>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		error()
	}
	fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		error()
	}
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		error()
	}
	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		error()
	}
	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		error()
	}
	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		error()
	}
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		error()
	}
	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		error()
	}
	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		error()
	}
}
