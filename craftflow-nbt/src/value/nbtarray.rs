use super::nbtseq::{NbtSeq, NbtSizedSeq};
use crate::ParseWrite;
use crate::Result;
use bytes::Bytes;
use std::io::{self, Write};

macro_rules! i_hate_naming_macros {
	($name:ident, $elem_type:ty) => {
		pub struct $name(NbtSeq<$elem_type>);

		impl $name {
			pub fn from_slice(data: impl AsRef<[$elem_type]>) -> Self {
				Self(NbtSeq::from_slice(data))
			}
			pub fn from_vec(data: Vec<$elem_type>) -> Self {
				Self(NbtSeq::from_vec(data))
			}
			pub fn as_slice(&self) -> &[$elem_type] {
				self.0.as_slice()
			}
		}

		impl ParseWrite for $name {
			fn parse(data: &mut Bytes) -> Result<Self> {
				NbtSeq::<$elem_type>::parse(data).map(Self)
			}
			fn write(&self, output: impl Write) -> io::Result<usize> {
				self.0.write(output)
			}
		}
	};
}
i_hate_naming_macros!(NbtByteArray, i8);
i_hate_naming_macros!(NbtIntArray, i32);
i_hate_naming_macros!(NbtLongArray, i64);
