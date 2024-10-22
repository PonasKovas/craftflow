use super::any::AnyDeserializer;
use crate::{tag::Tag, Error};
use serde::de::{DeserializeSeed, SeqAccess};

pub struct SeqDeserializer<'a, 'de> {
	input: &'a mut &'de [u8],
	tag: Tag,
	len: usize,
	index: usize,
}

impl<'a, 'de> SeqDeserializer<'a, 'de> {
	pub fn new(input: &'a mut &'de [u8], tag: Tag, len: usize) -> Self {
		Self {
			input,
			tag,
			len,
			index: 0,
		}
	}
}

impl<'a, 'de> SeqAccess<'de> for SeqDeserializer<'a, 'de> {
	type Error = Error;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		if self.index >= self.len {
			return Ok(None);
		}
		self.index += 1;

		let mut serializer = AnyDeserializer {
			input: *self.input,
			tag: Some(self.tag),
		};
		let r = seed.deserialize(&mut serializer);
		*self.input = serializer.input;
		r.map(Some)
	}
}
