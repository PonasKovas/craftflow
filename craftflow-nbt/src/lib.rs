#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![cfg_attr(feature = "nightly", feature(portable_simd))]

mod error;
/// exposed for macros
#[doc(hidden)]
pub mod internal;
mod r#macro;
mod nbtvalue;
mod tag;

use internal::{InternalNbtRead, InternalNbtWrite, read::read_tag, write::write_tag};

pub use craftflow_nbt_derive::Nbt;
pub use error::{Error, Result};
pub use nbtvalue::{
	NbtByteArray, NbtCompound, NbtIntArray, NbtList, NbtLongArray, NbtStr, NbtString, NbtValue,
};
pub use tag::Tag;

/// The main trait that allows to write NBT data.
pub trait NbtWrite {
	fn nbt_write(&self, output: &mut Vec<u8>) -> usize;
	fn nbt_write_named(&self, name: &NbtStr, output: &mut Vec<u8>) -> usize;
}

/// The main trait that allows to read NBT data.
pub trait NbtRead: Sized {
	fn nbt_read(input: &mut &[u8]) -> Result<Self>;
	fn nbt_read_named(input: &mut &[u8]) -> Result<(NbtString, Self)>;
}

impl<T: InternalNbtWrite> NbtWrite for T {
	fn nbt_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(T::TAG, output);
		written += self.nbt_iwrite(output);

		written
	}
	fn nbt_write_named(&self, name: &NbtStr, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(T::TAG, output);
		written += name.nbt_iwrite(output);
		written += self.nbt_iwrite(output);

		written
	}
}
impl<T: InternalNbtRead> NbtRead for T {
	fn nbt_read(input: &mut &[u8]) -> Result<Self> {
		let tag = read_tag(input)?;

		if tag != T::TAG {
			return Err(Error::UnexpectedTag(tag));
		}

		T::nbt_iread(input)
	}
	fn nbt_read_named(input: &mut &[u8]) -> Result<(NbtString, Self)> {
		let tag = read_tag(input)?;

		if tag != T::TAG {
			return Err(Error::UnexpectedTag(tag));
		}

		let name = NbtString::nbt_iread(input)?;

		T::nbt_iread(input).map(|v| (name, v))
	}
}
