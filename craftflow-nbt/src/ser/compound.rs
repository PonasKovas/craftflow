use super::{any::AnySerializer, string::StrSerializer, tag::TagSerializer};
use crate::{tag::Tag, Error};
use serde::{
	ser::{SerializeMap, SerializeStruct, SerializeStructVariant},
	Serialize,
};
use std::io::Write;

/// A serializer that serializes a compound tag.
/// Does not include the opening tag, but does include the closing tag.
pub struct CompoundSerializer<W> {
	output: W,
	pub written: usize,
	/// holds the serialized string of the key, if serializing as map (keys and values separately)
	key: Option<Vec<u8>>,
}
impl<W: Write> CompoundSerializer<W> {
	pub fn new(output: W, written: usize) -> Self {
		Self {
			output,
			written,
			key: None,
		}
	}
}
impl<W: Write> SerializeMap for CompoundSerializer<W> {
	type Ok = usize;
	type Error = Error;

	fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
		if self.key.is_some() {
			return Err(Error::InvalidData(format!(
				"expected compound value, got key"
			)));
		}
		// serialize the key into buffer and store it
		// we will use it in the next call to serialize_value
		let mut key_buffer = Vec::new();
		key.serialize(StrSerializer {
			output: &mut key_buffer,
		})?;
		self.key = Some(key_buffer);

		Ok(())
	}
	fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
		let key = match self.key.take() {
			Some(k) => k,
			None => {
				return Err(Error::InvalidData(format!(
					"expected compound key, got value"
				)))
			}
		};

		let tag = value.serialize(TagSerializer)?;

		// first write the tag
		self.output.write_all(&[tag as u8])?;
		self.written += 1;
		// then write the key
		self.output.write_all(&key)?;
		self.written += key.len();
		// then write the value
		self.written += value.serialize(AnySerializer {
			output: &mut self.output,
			expecting: Some(tag),
		})?;

		Ok(())
	}
	fn end(mut self) -> Result<Self::Ok, Self::Error> {
		self.output.write_all(&[Tag::End as u8])?;
		Ok(self.written + 1)
	}

	fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
	where
		K: ?Sized + Serialize,
		V: ?Sized + Serialize,
	{
		let tag = value.serialize(TagSerializer)?;

		// first write the tag
		self.output.write_all(&[tag as u8])?;
		self.written += 1;
		// then write the key
		self.written += key.serialize(StrSerializer {
			output: &mut self.output,
		})?;
		// then write the value
		self.written += value.serialize(AnySerializer {
			output: &mut self.output,
			expecting: Some(tag),
		})?;

		Ok(())
	}
}
impl<W: Write> SerializeStruct for CompoundSerializer<W> {
	type Ok = usize;
	type Error = Error;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		SerializeMap::serialize_entry(self, key, value)
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeMap::end(self)
	}
}
impl<W: Write> SerializeStructVariant for CompoundSerializer<W> {
	type Ok = usize;
	type Error = Error;

	fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + Serialize,
	{
		SerializeMap::serialize_entry(self, key, value)
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeMap::end(self)
	}
}