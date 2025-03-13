use super::{InternalNbtWrite, swap_endian::swap_endian};
use crate::{
	NbtStr, NbtString, Tag,
	nbtvalue::{NbtByteArray, NbtCompound, NbtIntArray, NbtList, NbtLongArray, NbtValue},
};
use std::{collections::HashMap, ptr::copy_nonoverlapping};

pub fn write_tag(tag: Tag, output: &mut Vec<u8>) -> usize {
	output.push(tag as u8);
	1
}

pub fn write_str(s: &NbtStr, output: &mut Vec<u8>) -> usize {
	let encoded = simd_cesu8::encode(s);

	output.extend_from_slice(&(encoded.len() as u16).to_be_bytes());
	output.extend_from_slice(&encoded);

	encoded.len() + 2
}

pub fn write_value(value: &NbtValue, output: &mut Vec<u8>) -> usize {
	match value {
		NbtValue::Byte(v) => v.nbt_iwrite(output),
		NbtValue::Short(v) => v.nbt_iwrite(output),
		NbtValue::Int(v) => v.nbt_iwrite(output),
		NbtValue::Long(v) => v.nbt_iwrite(output),
		NbtValue::Float(v) => v.nbt_iwrite(output),
		NbtValue::Double(v) => v.nbt_iwrite(output),
		NbtValue::ByteArray(v) => v.nbt_iwrite(output),
		NbtValue::String(v) => v.nbt_iwrite(output),
		NbtValue::List(v) => v.nbt_iwrite(output),
		NbtValue::Compound(v) => v.nbt_iwrite(output),
		NbtValue::IntArray(v) => v.nbt_iwrite(output),
		NbtValue::LongArray(v) => v.nbt_iwrite(output),
	}
}

pub fn write_seq<T: InternalNbtWrite>(seq: &Vec<T>, output: &mut Vec<u8>) -> usize {
	let mut written = 0;

	written += (seq.len() as i32).nbt_iwrite(output);

	match T::IS_POD {
		false => {
			for element in seq {
				written += element.nbt_iwrite(output);
			}
		}
		true => {
			#[allow(non_snake_case)]
			let N = size_of::<T>();

			let bytes = seq.len() * N;

			output.reserve(bytes);
			let start_indice = output.len();
			unsafe {
				// SAFETY: output is definitely not overlapping with seq.
				let output_ptr = output.as_mut_ptr().offset(start_indice as isize);
				copy_nonoverlapping(seq.as_ptr() as *const u8, output_ptr, bytes);
				output.set_len(start_indice + bytes);
			}

			let slice = &mut output[start_indice..];
			swap_endian(slice, N);

			written += bytes;
		}
	}

	written
}

macro_rules! gen_impl_simple {
    ($($type:ty),*) => {$(
        impl InternalNbtWrite for $type {
			fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
				output.extend_from_slice(&self.to_be_bytes());

				size_of::<$type>()
			}
		}
    )*};
}
gen_impl_simple!(i8, i16, i32, i64, f32, f64);

impl InternalNbtWrite for NbtString {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
		write_str(self, output)
	}
}
impl InternalNbtWrite for NbtByteArray {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
		write_seq(self, output)
	}
}
impl InternalNbtWrite for NbtIntArray {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
		write_seq(self, output)
	}
}
impl InternalNbtWrite for NbtLongArray {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
		write_seq(self, output)
	}
}

impl InternalNbtWrite for NbtList {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(self.tag(), output);
		written += match self {
			NbtList::Byte(vec) => write_seq(vec, output),
			NbtList::Short(vec) => write_seq(vec, output),
			NbtList::Int(vec) => write_seq(vec, output),
			NbtList::Long(vec) => write_seq(vec, output),
			NbtList::Float(vec) => write_seq(vec, output),
			NbtList::Double(vec) => write_seq(vec, output),
			NbtList::ByteArray(vec) => write_seq(vec, output),
			NbtList::String(vec) => write_seq(vec, output),
			NbtList::List(vec) => write_seq(vec, output),
			NbtList::Compound(vec) => write_seq(vec, output),
			NbtList::IntArray(vec) => write_seq(vec, output),
			NbtList::LongArray(vec) => write_seq(vec, output),
		};

		written
	}
}

impl InternalNbtWrite for NbtCompound {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		for (k, v) in self {
			written += write_tag(v.tag(), output);
			written += k.nbt_iwrite(output);
			written += write_value(v, output);
		}

		written += write_tag(Tag::End, output);

		written
	}
}

impl<T: InternalNbtWrite> InternalNbtWrite for Vec<T> {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(T::TAG, output);
		written += write_seq(self, output);

		written
	}
}
impl<T: InternalNbtWrite> InternalNbtWrite for HashMap<NbtString, T> {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		for (k, v) in self {
			written += write_tag(T::TAG, output);
			written += k.nbt_iwrite(output);
			written += v.nbt_iwrite(output);
		}

		written += write_tag(Tag::End, output);

		written
	}
}
