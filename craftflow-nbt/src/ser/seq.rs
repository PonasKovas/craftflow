use super::{any::AnySerializer, tag::TagSerializer};
use crate::{tag::Tag, Error};
use serde::ser::{SerializeSeq, SerializeTuple};
use std::io::Write;

pub struct SeqSerializer<'a, W> {
	output: &'a mut W,
	written: usize,
	/// if set to no None, will be changed to the tag of the first element
	/// and additionally written as a prefix
	tag: Option<Tag>,
	length: Length,
	length_prefix_written: bool,
}

enum Length {
	Known(usize),
	/// The buffer is used in the case when the length of the sequence is unknown.
	/// It is used to store the data until the sequence is ended.
	/// And only then the data is written to the output, together with the counted length
	Unknown {
		buffer: Vec<u8>,
		counter: usize,
	},
}

impl<'a, W: Write> SeqSerializer<'a, W> {
	/// If tag is `None`, it will be decided based on the first element
	/// and written as a prefix.
	/// Length is written either way
	pub fn new(output: &'a mut W, written: usize, tag: Option<Tag>, length: Option<usize>) -> Self {
		Self {
			output,
			written,
			tag,
			length: Length::new(length),
			length_prefix_written: false,
		}
	}
}

impl Length {
	pub fn new(len: Option<usize>) -> Self {
		match len {
			Some(len) => Self::Known(len),
			None => Self::Unknown {
				buffer: Vec::new(),
				counter: 0,
			},
		}
	}
}

impl<'a, W: Write> SerializeSeq for SeqSerializer<'a, W> {
	type Ok = usize;
	type Error = Error;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		let tag = value.serialize(TagSerializer)?;

		match &self.tag {
			Some(seq_tag) => {
				// make sure the tag of the sequence is compatible with the tag of the element
				if !tag.compatible_with(seq_tag) {
					return Err(Error::InvalidData(format!(
						"found {tag} in sequence of {seq_tag} - incompatible"
					)));
				}
			}
			None => {
				// if tag not set yet, this is the first element
				// set the tag
				self.tag = Some(tag);

				// write the tag
				self.output.write_all(&[tag as u8])?;
				self.written += 1;
			}
		}

		// if length prefix not written yet, write it
		if !self.length_prefix_written {
			self.length_prefix_written = true;

			// but only if we know it
			if let &Length::Known(len) = &self.length {
				self.output.write_all(&(len as i32).to_be_bytes())?;
				self.written += 4;
			}
			// otherwise it will be written at the end of the sequence when we know it
		}

		match &mut self.length {
			Length::Known(_) => {
				// just write the element to the output
				let serializer = AnySerializer {
					output: &mut self.output,
					expecting: self.tag,
				};
				self.written += value.serialize(serializer)?;
			}
			Length::Unknown { buffer, counter } => {
				// Write to the buffer and increase the counter
				let serializer = AnySerializer {
					output: buffer,
					expecting: self.tag,
				};
				self.written += value.serialize(serializer)?;

				*counter += 1;
			}
		}

		Ok(())
	}
	fn end(mut self) -> Result<Self::Ok, Self::Error> {
		match self.length {
			Length::Known(len) => {
				if len == 0 {
					// normally writing the tag and length is handled in the serialize_element method
					// but if the length is 0, we have to write it here
					self.output.write_all(&[Tag::End as u8, 0, 0, 0, 0])?;
					self.written += 5;
				}
			}
			Length::Unknown { buffer, counter } => {
				if counter == 0 {
					// normally the tag is written in serialize_element method
					// but since there were no elements, we have to do it here
					self.output.write_all(&[Tag::End as u8])?;
					self.written += 1;
				}

				// write the length
				self.output.write_all(&(counter as i32).to_be_bytes())?;
				self.written += 5;

				// write the buffer
				self.output.write_all(&buffer)?;
			}
		}

		Ok(self.written)
	}
}
impl<'a, W: Write> SerializeTuple for SeqSerializer<'a, W> {
	type Ok = usize;
	type Error = Error;

	fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + serde::Serialize,
	{
		SerializeSeq::serialize_element(self, value)
	}
	fn end(self) -> Result<Self::Ok, Self::Error> {
		SerializeSeq::end(self)
	}
}
