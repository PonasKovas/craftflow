use super::{AbstrBytes, AbstrBytesMut, NbtGet, NbtValidate, NbtWrite};
use crate::{Error, Result};
use core::str;
use std::{borrow::Cow, ops::Deref};
use typenum::U0;

const PADDING: u8 = 0xC0;

pub(crate) struct StringBytes<B> {
	pub(crate) bytes: B,
}

impl<B: AbstrBytes> StringBytes<B> {
	pub fn as_str(&self) -> &str {
		unsafe { str::from_utf8(&self.bytes[..]).unwrap_unchecked() }
	}
}

impl<B: AbstrBytes> NbtValidate for StringBytes<B> {
	const IS_STATIC: bool = false;
	type StaticSize = U0;

	fn dynamic_validate<B2: AbstrBytesMut>(data: &mut B2) -> Result<B2::Immutable> {
		if data.len() < 2 {
			return Err(Error::InsufficientData(2 - data.len()));
		}
		let len = i16::from_be_bytes([data[0], data[1]]) as usize;
		data.advance(2);

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

		let bytes = data.split_bytes(len).freeze();

		Ok(bytes)
	}
}

impl<B: AbstrBytes> NbtGet<B> for StringBytes<B> {
	unsafe fn get(data: &mut B) -> Self {
		let len = unsafe { i16::get(data) } as usize;

		let mut buffer = data.split_bytes(len);

		// remove 0xC0 padding
		let real_length = buffer[..].iter().rposition(|b| *b != PADDING).unwrap_or(0);

		buffer.truncate(real_length);

		Self { bytes: buffer }
	}
}

impl<B: AbstrBytes> NbtWrite for StringBytes<B> {
	fn write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		// placeholder for length
		let len_pos = output.len();
		output.extend_from_slice(&[0u8; 2]);
		written += 2;

		// gotta convert to mutf8
		// TODO optimize by converting in place.
		let mutf8 = simd_cesu8::mutf8::encode(self.as_str());
		output.extend_from_slice(&mutf8);
		written += mutf8.len();

		let len = output.len() - len_pos - 2;
		output[len_pos..(len_pos + 2)].copy_from_slice(&(len as i16).to_be_bytes());

		written
	}
}

impl<T: AbstrBytes> Deref for StringBytes<T> {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}
