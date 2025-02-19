use super::super::NbtSeq;
use crate::{advance, nbt_format::NbtFormat, Error, NbtString, Result};
use bytes::{Buf, Bytes};

impl<T: NbtFormat> NbtFormat for NbtSeq<NbtSeq<T>> {
	fn validate(data: &mut &mut [u8]) -> Result<()> {
		if data.len() < 4 {
			return Err(Error::InsufficientData(4 - data.len()));
		}
		let len = i32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
		advance(data, 4);

		for _ in 0..len {
			NbtSeq::<T>::validate(data)?;
		}

		Ok(())
	}
	unsafe fn get(data: &mut Bytes) -> Self {
		let n_bytes = Self::count_bytes(&data[..]);

		let mut buf = data.split_to(n_bytes);
		let len = buf.get_i32() as usize;

		Self::new_raw(buf, len as usize)
	}
	unsafe fn count_bytes(mut data: &[u8]) -> usize {
		let len = i32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
		data = &data[4..];

		let mut bytes = 4;
		for _ in 0..len {
			bytes += NbtString::count_bytes(data);
		}

		bytes
	}
	fn write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += (self.len as i32).write(output);

		// idk if this is very efficient, since it involves atomic operations when splitting/cloning Bytes
		// but to make it better i would need to duplicate these types using slices instead of Bytes
		// so idk
		for s in self.iter() {
			written += s.write(output);
		}

		written
	}
}
