use crate::{advance, nbt_format::NbtFormat, Error, Result};
use bytes::{Buf, Bytes};
use core::str;
use std::{borrow::Cow, ops::Deref};

const PADDING: u8 = 0xC0;

#[derive(Debug, PartialEq, Clone)]
pub struct NbtString {
	// only the string data in UTF-8, not including length or trailing padding
	pub(crate) data: Bytes,
}

impl NbtString {
	pub fn from_str(data: impl AsRef<str>) -> Self {
		Self::from_string(data.as_ref().to_string())
	}
	pub fn from_string(data: String) -> Self {
		// just save the utf8 string as Bytes
		Self::new(Bytes::from(data))
	}
	pub fn as_str(&self) -> &str {
		unsafe { str::from_utf8(&self.data[..]).unwrap_unchecked() }
	}
}

impl NbtFormat for NbtString {
	unsafe fn get(data: &mut Bytes) -> Self {
		let len = data.get_i16() as usize;

		let mut buffer = data.split_to(len);

		// remove 0xC0 padding
		let real_length = buffer[..].iter().rposition(|b| *b != PADDING).unwrap_or(0);
		// this will fail if 0-len str, so fallback to 0. otherwise this cant happen, because its
		// validated to be a valid utf8 str, and 0xC0 is not valid in utf8.
		buffer.truncate(real_length);

		Self::new(buffer)
	}
	fn write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		let len = self.data.len();
		written += (len as i16).write(output);

		// gotta convert to mutf8
		// TODO optimize by converting in place.
		let mutf8 = simd_cesu8::mutf8::encode(&*self.as_str());
		output.extend_from_slice(&mutf8);
		written += mutf8.len();

		written
	}
	fn validate(data: &mut &mut [u8]) -> Result<()> {
		if data.len() < 2 {
			return Err(Error::InsufficientData(2 - data.len()));
		}
		let len = i16::from_be_bytes([data[0], data[1]]) as usize;
		advance(data, 2);

		if data.len() < len {
			return Err(Error::InsufficientData(len - data.len()));
		}

		// gotta validate that it's good mutf8 and convert to utf8 in place
		let utf8 = simd_cesu8::mutf8::decode_strict(&data[..len])?;

		// clever optimization here.
		// we know that utf8 is never longer than its equivalent mutf8
		// so utf8 will always fit in the original buffer, but it can be shorter
		// so we will just replace the string data with utf8 in place in the original buffer
		// and add padding to the end to avoid shifting all of the unrelated bytes that come after
		// the padding is of 0xCO bytes, which are ALWAYS invalid in utf8, so when reading NbtString
		// optimized for reading, we also have to remove any trailing 0xC0 bytes
		// check get() method for this
		if let Cow::Owned(utf8) = utf8 {
			// only if owned, bcs if borrowed, no need to change anything anyway
			data[..utf8.len()].copy_from_slice(utf8.as_bytes());
			data[utf8.len()..len].fill(PADDING);
		}

		Ok(())
	}
	unsafe fn count_bytes(data: &[u8]) -> usize {
		let len = i16::from_be_bytes([data[0], data[1]]) as usize;

		len + 2
	}
}

impl Deref for NbtString {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl NbtString {
	fn new(data: Bytes) -> Self {
		Self { data }
	}
}
