use super::{BytesAbstr, BytesMutAbstr, NbtBytes};
use crate::{Error, Result};
use bytes::{Buf, Bytes};
use core::str;
use std::{borrow::Cow, ops::Deref};
use typenum::U0;

const PADDING: u8 = 0xC0;

pub(crate) struct StringBytes<T> {
	pub(crate) bytes: T,
}

impl<T: BytesAbstr> StringBytes<T> {
	pub fn as_str(&self) -> &str {
		unsafe { str::from_utf8(&self.bytes[..]).unwrap_unchecked() }
	}
}

impl<T: BytesAbstr> NbtBytes<T> for StringBytes<T> {
	type ConstSize = U0; // not statically sized

	fn validate<B: BytesMutAbstr<Immutable = T>>(data: &mut B) -> Result<Self> {
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

		let mut bytes = data.split_chunk(len).freeze();

		Ok(unsafe { Self::new(&mut bytes) })
	}
	unsafe fn new(data: &mut T) -> Self {
		let len = unsafe { i16::new(data) } as usize;

		let mut buffer = data.split_chunk(len);

		// remove 0xC0 padding
		let real_length = buffer[..].iter().rposition(|b| *b != PADDING).unwrap_or(0);

		buffer.truncate(real_length);

		Self { bytes: buffer }
	}
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

impl<T: BytesAbstr> Deref for StringBytes<T> {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}
