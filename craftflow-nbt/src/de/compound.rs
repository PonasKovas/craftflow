use super::any::AnyDeserializer;
use crate::{de::read_ext::ByteRead, tag::Tag, Error};
use serde::de::{DeserializeSeed, MapAccess};

pub struct CompoundDeserializer<'a, 'de> {
	input: &'a mut &'de [u8],
	tag: Option<Tag>, // is set when key is read, reset when value read
}

impl<'a, 'de> CompoundDeserializer<'a, 'de> {
	pub fn new(input: &'a mut &'de [u8]) -> Self {
		Self { input, tag: None }
	}
}

impl<'a, 'de> MapAccess<'de> for CompoundDeserializer<'a, 'de> {
	type Error = Error;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: DeserializeSeed<'de>,
	{
		let tag = Tag::new(self.input.read_u8()?)?;
		if tag == Tag::End {
			return Ok(None);
		}
		self.tag = Some(tag);

		let mut serializer = AnyDeserializer {
			input: self.input,
			tag: Some(Tag::String),
		};
		let r = seed.deserialize(&mut serializer);

		*self.input = serializer.input;
		r.map(Some)
	}
	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		println!("DESERIALIZING VALUE");
		let tag = self
			.tag
			.take()
			.ok_or(Error::InvalidData(format!("compound value without key")))?;

		let mut serializer = AnyDeserializer {
			input: self.input,
			tag: Some(tag),
		};
		let r = seed.deserialize(&mut serializer);
		*self.input = serializer.input;
		r
	}
}
