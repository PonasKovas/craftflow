use super::{compound::CompoundSerializer, seq::SeqSerializer, write_str::write_str};
use crate::{
	arrays::{MAGIC_BYTE_ARRAY, MAGIC_INT_ARRAY, MAGIC_LONG_ARRAY},
	tag::Tag,
	Error,
};
use serde::{ser::Impossible, Serialize, Serializer};
use std::io::Write;

/// Serializes any value, allows to only accept a specific type
pub struct AnySerializer<'a, W> {
	pub output: &'a mut W,
	/// If Some, will accept only the given tag and will not write the tag itself
	/// This is used in lists
	pub expecting: Option<Tag>,
}

impl<'a, W: Write> Serializer for AnySerializer<'a, W> {
	type Ok = usize;
	type Error = Error;
	type SerializeSeq = SeqSerializer<'a, W>;
	type SerializeTuple = SeqSerializer<'a, W>;
	type SerializeTupleStruct = Impossible<usize, Error>;
	type SerializeTupleVariant = Impossible<usize, Error>;
	type SerializeMap = CompoundSerializer<'a, W>;
	type SerializeStruct = CompoundSerializer<'a, W>;
	type SerializeStructVariant = CompoundSerializer<'a, W>;

	///////////////////////////////////////////
	// Maps
	//

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		let mut written = 0;
		match self.expecting {
			None => {
				self.output.write_all(&[Tag::Compound as u8])?;
				written += 1;
			}
			Some(expecting) => match expecting {
				Tag::Compound => {}
				other => {
					return Err(Error::InvalidData(format!(
						"expected {other}, found compound"
					)))
				}
			},
		}

		Ok(CompoundSerializer::new(self.output, written))
	}
	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		self.serialize_map(None)
	}
	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		self.serialize_map(None)
	}

	///////////////////////////////////////////
	// Sequences
	//

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		let mut written = 0;
		let mut element_tag = None;
		match self.expecting {
			None => {
				self.output.write_all(&[Tag::List as u8])?;
				written += 1;
				element_tag = None; // we dont know the type of the elements
			}
			Some(expecting) => match expecting {
				Tag::List => {}
				// for the following arrays we know the type of the element up front
				// and dont need to write it explicitly
				Tag::ByteArray => {
					element_tag = Some(Tag::Byte);
				}
				Tag::IntArray => {
					element_tag = Some(Tag::Int);
				}
				Tag::LongArray => {
					element_tag = Some(Tag::Long);
				}
				other => return Err(Error::InvalidData(format!("expected {other}, found list"))),
			},
		};

		Ok(SeqSerializer::new(self.output, written, element_tag, len))
	}
	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		// for whatever reason serde uses this for all statically-known size sequences too
		// instead of just fucking using seq with Some(len) so we have to implement this
		// but NBT does not support tuples with different types
		self.serialize_seq(Some(len))
	}

	///////////////////////////////////////////
	// Normal types
	//

	fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
		let mut written = 0;
		match self.expecting {
			None => {
				self.output.write_all(&[Tag::String as u8])?;
				written += 1;
			}
			Some(expecting) => match expecting {
				Tag::String => {}
				other => {
					return Err(Error::InvalidData(format!(
						"expected {other}, found string"
					)))
				}
			},
		}

		Ok(write_str(&mut self.output, v)? + written)
	}
	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		let mut written = 0;
		let mut length = v.len() as i32;
		match self.expecting {
			None => {
				// Default as List (use the newtype wrappers for ByteArray, IntArray, LongArray)
				self.output.write_all(&[Tag::List as u8])?;
				self.output.write_all(&[Tag::Byte as u8])?;
				written += 2;
			}
			Some(expecting) => match expecting {
				Tag::List => {
					self.output.write_all(&[Tag::Byte as u8])?;
					written += 1;
				}
				Tag::ByteArray => {}
				Tag::IntArray => {
					if length % 4 != 0 {
						return Err(Error::InvalidData(format!("byte array length must be a multiple of 4, to be serialized as IntArray")));
					}
					length /= 4;
				}
				Tag::LongArray => {
					if length % 8 != 0 {
						return Err(Error::InvalidData(format!("byte array length must be a multiple of 8, to be serialized as IntArray")));
					}
					length /= 8;
				}
				other => {
					return Err(Error::InvalidData(format!(
						"expected {other}, found byte array"
					)))
				}
			},
		}

		self.output.write_all(&length.to_be_bytes())?;
		written += 4;

		self.output.write_all(v)?;
		written += v.len();

		Ok(written)
	}

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		let written = match self.expecting {
			None => {
				self.output.write_all(&[Tag::Byte as u8])?;
				self.output.write_all(&(v as u8).to_be_bytes())?;
				2
			}
			Some(expecting) => match expecting {
				Tag::Byte => {
					self.output.write_all(&(v as u8).to_be_bytes())?;
					1
				}
				Tag::Short => {
					self.output.write_all(&(v as u16).to_be_bytes())?;
					2
				}
				Tag::Int => {
					self.output.write_all(&(v as u32).to_be_bytes())?;
					4
				}
				Tag::Long => {
					self.output.write_all(&(v as u64).to_be_bytes())?;
					8
				}
				other => return Err(Error::InvalidData(format!("expected {other}, found bool"))),
			},
		};

		Ok(written)
	}
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		let written = match self.expecting {
			None => {
				self.output.write_all(&[Tag::Byte as u8])?;
				self.output.write_all(&v.to_be_bytes())?;
				2
			}
			Some(expecting) => match expecting {
				Tag::Byte => {
					self.output.write_all(&(v as i8).to_be_bytes())?;
					1
				}
				Tag::Short => {
					self.output.write_all(&(v as i16).to_be_bytes())?;
					2
				}
				Tag::Int => {
					self.output.write_all(&(v as i32).to_be_bytes())?;
					4
				}
				Tag::Long => {
					self.output.write_all(&(v as i64).to_be_bytes())?;
					8
				}
				other => return Err(Error::InvalidData(format!("expected {other}, found byte"))),
			},
		};

		Ok(written)
	}
	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		let written = match self.expecting {
			None => {
				self.output.write_all(&[Tag::Short as u8])?;
				self.output.write_all(&v.to_be_bytes())?;
				3
			}
			Some(expecting) => match expecting {
				Tag::Short => {
					self.output.write_all(&(v as i16).to_be_bytes())?;
					2
				}
				Tag::Int => {
					self.output.write_all(&(v as i32).to_be_bytes())?;
					4
				}
				Tag::Long => {
					self.output.write_all(&(v as i64).to_be_bytes())?;
					8
				}
				other => return Err(Error::InvalidData(format!("expected {other}, found short"))),
			},
		};

		Ok(written)
	}
	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		let written = match self.expecting {
			None => {
				self.output.write_all(&[Tag::Int as u8])?;
				self.output.write_all(&v.to_be_bytes())?;
				5
			}
			Some(expecting) => match expecting {
				Tag::Int => {
					self.output.write_all(&(v as i32).to_be_bytes())?;
					4
				}
				Tag::Long => {
					self.output.write_all(&(v as i64).to_be_bytes())?;
					8
				}
				other => return Err(Error::InvalidData(format!("expected {other}, found int"))),
			},
		};

		Ok(written)
	}
	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		let written = match self.expecting {
			None => {
				self.output.write_all(&[Tag::Long as u8])?;
				self.output.write_all(&v.to_be_bytes())?;
				9
			}
			Some(expecting) => match expecting {
				Tag::Long => {
					self.output.write_all(&(v as i64).to_be_bytes())?;
					8
				}
				other => return Err(Error::InvalidData(format!("expected {other}, found long"))),
			},
		};

		Ok(written)
	}
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		self.serialize_i8(v as i8)
	}
	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		self.serialize_i16(v as i16)
	}
	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		self.serialize_i32(v as i32)
	}
	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		self.serialize_i64(v as i64)
	}
	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		let written = match self.expecting {
			None => {
				self.output.write_all(&[Tag::Float as u8])?;
				self.output.write_all(&v.to_be_bytes())?;
				5
			}
			Some(expecting) => match expecting {
				Tag::Float => {
					self.output.write_all(&(v as f32).to_be_bytes())?;
					4
				}
				Tag::Double => {
					self.output.write_all(&(v as f64).to_be_bytes())?;
					8
				}
				other => return Err(Error::InvalidData(format!("expected {other}, found float"))),
			},
		};

		Ok(written)
	}
	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		let written = match self.expecting {
			None => {
				self.output.write_all(&[Tag::Double as u8])?;
				self.output.write_all(&v.to_be_bytes())?;
				9
			}
			Some(expecting) => match expecting {
				Tag::Double => {
					self.output.write_all(&(v as f64).to_be_bytes())?;
					8
				}
				other => {
					return Err(Error::InvalidData(format!(
						"expected {other}, found double"
					)))
				}
			},
		};

		Ok(written)
	}
	fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
		// too ambiguous to be implemented
		// should this be serialized as a string, as a byte or as a u32?

		Err(Error::InvalidData(format!("char is not supported")))
	}
	fn serialize_tuple_struct(
		self,
		name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Err(Error::InvalidData(format!("{name}: tuples not supported")))
	}
	fn serialize_tuple_variant(
		self,
		name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Err(Error::InvalidData(format!("{name}: tuples not supported")))
	}
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		self.output.write_all(&[Tag::End as u8])?;
		Ok(1)
	}
	fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		value.serialize(self)
	}
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Ok(0)
	}
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}
	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
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

	// This method is also used to indicate that the sequence wrapped should be in a specific format
	// ByteArray, IntArray or LongArray
	fn serialize_newtype_struct<T>(
		self,
		name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: ?Sized + Serialize,
	{
		// If the newtype name matches any of the magic names, enable special behaviour
		let tag = match name {
			MAGIC_BYTE_ARRAY => Tag::ByteArray,
			MAGIC_INT_ARRAY => Tag::IntArray,
			MAGIC_LONG_ARRAY => Tag::LongArray,
			_ => return value.serialize(self),
		};

		let mut written = 0;
		match self.expecting {
			None => {
				self.output.write_all(&[tag as u8])?;
				written += 1;
			}
			Some(expecting) => {
				if expecting != tag {
					return Err(Error::InvalidData(format!(
						"expected {expecting}, found {tag}"
					)));
				}
			}
		}

		let serializer = AnySerializer {
			output: self.output,
			expecting: Some(tag),
		};

		Ok(value.serialize(serializer)? + written)
	}
}
