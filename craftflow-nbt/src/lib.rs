#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

mod error;
mod internal;
mod nbtvalue;
mod tag;
// #[cfg(test)]
// pub(crate) mod tests;

pub use error::{Error, Result};
use internal::{
	read::read_tag,
	write::{write_str, write_tag},
	InternalNbtRead, InternalNbtWrite,
};

pub trait Nbt: Sized {
	fn nbt_write(&self, output: &mut Vec<u8>) -> usize;
	fn nbt_write_named(&self, name: &str, output: &mut Vec<u8>) -> usize;
	fn nbt_read(input: &mut &[u8]) -> Result<Self>;
	fn nbt_read_named(input: &mut &[u8]) -> Result<(String, Self)>;
}

impl<T: InternalNbtRead + InternalNbtWrite> Nbt for T {
	fn nbt_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(T::TAG, output);
		written += self.nbt_iwrite(output);

		written
	}
	fn nbt_write_named(&self, name: &str, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(T::TAG, output);
		written += write_str(name, output);
		written += self.nbt_iwrite(output);

		written
	}
	fn nbt_read(input: &mut &[u8]) -> Result<Self> {
		let tag = read_tag(input)?;

		if tag != T::TAG {
			return Err(Error::UnexpectedTag(tag));
		}

		T::nbt_iread(input)
	}
	fn nbt_read_named(input: &mut &[u8]) -> Result<(String, Self)> {
		let tag = read_tag(input)?;

		if tag != T::TAG {
			return Err(Error::UnexpectedTag(tag));
		}

		let name = String::nbt_iread(input)?;

		T::nbt_iread(input).map(|v| (name, v))
	}
}
