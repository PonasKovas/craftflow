//! For deserializing types like ByteArray, IntArray and LongArray we use a fake enum
//! to tell the deserialize structure what type of array we are have.

use super::any::AnyDeserializer;
use crate::{
	arrays::{MAGIC_BYTE_ARRAY, MAGIC_INT_ARRAY, MAGIC_LONG_ARRAY},
	tag::Tag,
	Error,
};
use serde::{
	de::{EnumAccess, VariantAccess},
	Deserializer,
};

pub struct ArrayTypeEnumAccess<'a, 'de> {
	pub input: &'a mut &'de [u8],
	pub tag: Tag,
}
struct ArrayTypeTagDeserializer(&'static str);

impl<'a, 'de> EnumAccess<'de> for ArrayTypeEnumAccess<'a, 'de> {
	type Error = Error;
	type Variant = Self;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: serde::de::DeserializeSeed<'de>,
	{
		Ok((
			seed.deserialize(ArrayTypeTagDeserializer(match self.tag {
				Tag::ByteArray => MAGIC_BYTE_ARRAY,
				Tag::IntArray => MAGIC_INT_ARRAY,
				Tag::LongArray => MAGIC_LONG_ARRAY,
				_ => unreachable!(),
			}))?,
			self,
		))
	}
}

impl<'a, 'de> VariantAccess<'de> for ArrayTypeEnumAccess<'a, 'de> {
	type Error = Error;

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: serde::de::DeserializeSeed<'de>,
	{
		seed.deserialize(&mut AnyDeserializer {
			input: self.input,
			tag: Some(self.tag),
		})
	}

	fn unit_variant(self) -> Result<(), Self::Error> {
		Err(Error::InvalidData("array must be newtype".to_string()))
	}
	fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::InvalidData("array must be newtype".to_string()))
	}
	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		_visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		Err(Error::InvalidData("array must be newtype".to_string()))
	}
}

impl<'de> Deserializer<'de> for ArrayTypeTagDeserializer {
	type Error = crate::Error;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: serde::de::Visitor<'de>,
	{
		visitor.visit_borrowed_str(self.0)
	}
	serde::forward_to_deserialize_any! {bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
	bytes byte_buf option unit unit_struct newtype_struct seq tuple
	tuple_struct map struct enum identifier ignored_any }
}
