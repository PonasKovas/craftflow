use super::{compound::CompoundDeserializer, read_ext::ByteRead, seq::SeqDeserializer};
use crate::{
	arrays::{MAGIC_BYTE_ARRAY, MAGIC_INT_ARRAY, MAGIC_LONG_ARRAY},
	tag::Tag,
	Error,
};
use serde::{de::Visitor, Deserializer};
use std::borrow::Cow;

pub struct AnyDeserializer<'de> {
	pub input: &'de [u8],
	/// If tag is already known and doesnt need to be read (for example in lists)
	pub tag: Option<Tag>,
}

impl<'de> AnyDeserializer<'de> {
	fn tag(&mut self) -> crate::Result<Tag> {
		if let Some(tag) = self.tag {
			Ok(tag)
		} else {
			let tag = Tag::new(self.input.read_u8()?)?;
			self.tag = Some(tag);
			Ok(tag)
		}
	}
}

impl<'a, 'de> Deserializer<'de> for &'a mut AnyDeserializer<'de> {
	type Error = Error;

	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;

		match tag {
			Tag::End => self.deserialize_option(visitor),
			Tag::Compound => self.deserialize_map(visitor),
			Tag::List | Tag::ByteArray | Tag::IntArray | Tag::LongArray => {
				self.deserialize_seq(visitor)
			}
			Tag::String => self.deserialize_str(visitor),
			Tag::Byte => self.deserialize_i8(visitor),
			Tag::Short => self.deserialize_i16(visitor),
			Tag::Int => self.deserialize_i32(visitor),
			Tag::Long => self.deserialize_i64(visitor),
			Tag::Float => self.deserialize_f32(visitor),
			Tag::Double => self.deserialize_f64(visitor),
		}
	}
	fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Byte {
			return Err(Error::InvalidData(format!(
				"Expected byte tag for bool, found {tag}"
			)));
		}
		visitor.visit_bool(self.input.read_u8()? != 0)
	}
	fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Byte {
			return Err(Error::InvalidData(format!(
				"Expected byte tag for i8, found {tag}"
			)));
		}
		visitor.visit_i8(self.input.read_u8()? as i8)
	}
	fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Short {
			return Err(Error::InvalidData(format!(
				"Expected short tag for i16, found {tag}"
			)));
		}
		visitor.visit_i16(self.input.read_u16()? as i16)
	}
	fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Int {
			return Err(Error::InvalidData(format!(
				"Expected int tag for i32, found {tag}"
			)));
		}
		visitor.visit_i32(self.input.read_u32()? as i32)
	}
	fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Long {
			return Err(Error::InvalidData(format!(
				"Expected long tag for i64, found {tag}"
			)));
		}
		visitor.visit_i64(self.input.read_u64()? as i64)
	}
	fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Byte {
			return Err(Error::InvalidData(format!(
				"Expected byte tag for u8, found {tag}"
			)));
		}
		visitor.visit_u8(self.input.read_u8()?)
	}
	fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Short {
			return Err(Error::InvalidData(format!(
				"Expected short tag for u16, found {tag}"
			)));
		}
		visitor.visit_u16(self.input.read_u16()?)
	}
	fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Int {
			return Err(Error::InvalidData(format!(
				"Expected int tag for u32, found {tag}"
			)));
		}
		visitor.visit_u32(self.input.read_u32()?)
	}
	fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Long {
			return Err(Error::InvalidData(format!(
				"Expected long tag for u64, found {tag}"
			)));
		}
		visitor.visit_u64(self.input.read_u64()?)
	}
	fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Float {
			return Err(Error::InvalidData(format!(
				"Expected float tag for f32, found {tag}"
			)));
		}
		visitor.visit_f32(f32::from_bits(self.input.read_u32()?))
	}
	fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Double {
			return Err(Error::InvalidData(format!(
				"Expected double tag for f64, found {tag}"
			)));
		}
		visitor.visit_f64(f64::from_bits(self.input.read_u64()?))
	}
	fn deserialize_char<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
		Err(Error::InvalidData(format!("nbt does not support char")))
	}
	fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::String {
			return Err(Error::InvalidData(format!(
				"Expected string tag, found {tag}"
			)));
		}

		match self.input.read_str()? {
			Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
			Cow::Owned(s) => visitor.visit_string(s),
		}
	}
	fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_str(visitor)
	}
	fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_seq(visitor)
	}
	fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_bytes(visitor)
	}
	fn deserialize_newtype_struct<V: Visitor<'de>>(
		self,
		name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		// normally we ignore newtype structs in this format
		// but if it has one of the magic names
		// we enforce the data type
		let expected_tag = match name {
			MAGIC_BYTE_ARRAY => Tag::ByteArray,
			MAGIC_INT_ARRAY => Tag::IntArray,
			MAGIC_LONG_ARRAY => Tag::LongArray,
			_ => return visitor.visit_newtype_struct(self),
		};

		let tag = self.tag()?;
		if tag != expected_tag {
			return Err(Error::InvalidData(format!(
				"Expected {expected_tag} tag, found {tag}"
			)));
		}

		visitor.visit_newtype_struct(self)
	}
	fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;

		let element_tag = match tag {
			Tag::List => Tag::new(self.input.read_u8()?)?,
			Tag::ByteArray => Tag::Byte,
			Tag::IntArray => Tag::Int,
			Tag::LongArray => Tag::Long,
			_ => {
				return Err(Error::InvalidData(format!(
					"Expected list/bytearray/intarray/longarray tag, found {tag}"
				)))
			}
		};

		let mut len = self.input.read_u32()? as i32;
		if len < 0 {
			len = 0;
		}
		visitor.visit_seq(SeqDeserializer::new(self.input, element_tag, len as usize))
	}
	fn deserialize_tuple<V: Visitor<'de>>(
		self,
		_len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		self.deserialize_seq(visitor)
	}
	fn deserialize_tuple_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		_len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		self.deserialize_seq(visitor)
	}
	fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag != Tag::Compound {
			return Err(Error::InvalidData(format!(
				"Expected compound tag, found {tag}"
			)));
		}
		visitor.visit_map(CompoundDeserializer::new(self.input))
	}
	fn deserialize_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		self.deserialize_map(visitor)
	}
	fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		visitor.visit_unit()
	}
	fn deserialize_unit_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		visitor.visit_unit()
	}
	fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		let tag = self.tag()?;
		if tag == Tag::End {
			visitor.visit_none()
		} else {
			visitor.visit_some(self)
		}
	}
	fn deserialize_enum<V: Visitor<'de>>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Self::Error> {
		Err(Error::InvalidData(format!(
			"nbt does not support enums. Use #[serde(untagged)] if its meant to be untagged"
		)))
	}
	fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_any(visitor)
	}
	fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_any(visitor)
	}
}
