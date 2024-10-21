use super::{any::AnySerializer, compound_key::CompoundKeySerializer, tag::TagSerializer};
use crate::{tag::Tag, Error};
use serde::{
	ser::{SerializeMap, SerializeStruct, SerializeStructVariant},
	Serialize,
};
use std::io::Write;

/// A serializer that serializes a compound tag.
/// Does not include the opening tag, but does include the closing tag.
pub struct CompoundSerializer<'a, W> {
	output: &'a mut W,
	pub written: usize,
	/// holds the serialized string of the key, if serializing as map (keys and values separately)
	key: Option<Vec<u8>>,
}
impl<'a, W: Write> CompoundSerializer<'a, W> {
	pub fn new(output: &'a mut W, written: usize) -> Self {
		Self {
			output,
			written,
			key: None,
		}
	}
}
impl<'a, W: Write> SerializeMap for CompoundSerializer<'a, W> {
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
		key.serialize(CompoundKeySerializer {
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

		let tag = match value.serialize(TagSerializer)? {
			Some(t) => t,
			None => {
				// if there is no tag, that means the value can not be serialized, so we just skip this entry
				return Ok(());
			}
		};

		// first write the tag
		self.output.write_all(&[tag as u8])?;
		self.written += 1;
		// then write the key
		self.output.write_all(&key)?;
		self.written += key.len();
		// then write the value
		self.written += value.serialize(AnySerializer {
			output: self.output,
			expecting: Some(tag),
		})?;

		Ok(())
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		self.output.write_all(&[Tag::End as u8])?;
		Ok(self.written + 1)
	}

	fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
	where
		K: ?Sized + Serialize,
		V: ?Sized + Serialize,
	{
		let tag = match value.serialize(TagSerializer)? {
			Some(t) => t,
			None => {
				// if there is no tag, that means the value can not be serialized, so we just skip this entry
				return Ok(());
			}
		};

		// first write the tag
		self.output.write_all(&[tag as u8])?;
		self.written += 1;
		// then write the key
		self.written += key.serialize(CompoundKeySerializer {
			output: &mut self.output,
		})?;
		// then write the value
		self.written += value.serialize(AnySerializer {
			output: self.output,
			expecting: Some(tag),
		})?;

		Ok(())
	}
}
impl<'a, W: Write> SerializeStruct for CompoundSerializer<'a, W> {
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
impl<'a, W: Write> SerializeStructVariant for CompoundSerializer<'a, W> {
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
