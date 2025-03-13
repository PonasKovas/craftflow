use super::InternalNbtRead;
use crate::{
	nbtvalue::{NbtByteArray, NbtCompound, NbtIntArray, NbtList, NbtLongArray, NbtValue},
	Error, Result, Tag,
};
use std::{
	collections::{hash_map::Entry, HashMap},
	ptr::copy_nonoverlapping,
	slice,
};
use typenum::Unsigned;

pub fn read_tag(input: &mut &[u8]) -> Result<Tag> {
	if input.len() < 1 {
		return Err(Error::NotEnoughData(1));
	}

	let tag = Tag::new(input[0])?;
	advance(input, 1);

	Ok(tag)
}

pub fn read_value(input: &mut &[u8], tag: Tag) -> Result<NbtValue> {
	let val = match tag {
		Tag::End => return Err(Error::InvalidTag(0)),
		Tag::Byte => NbtValue::Byte(i8::nbt_iread(input)?),
		Tag::Short => NbtValue::Short(i16::nbt_iread(input)?),
		Tag::Int => NbtValue::Int(i32::nbt_iread(input)?),
		Tag::Long => NbtValue::Long(i64::nbt_iread(input)?),
		Tag::Float => NbtValue::Float(f32::nbt_iread(input)?),
		Tag::Double => NbtValue::Double(f64::nbt_iread(input)?),
		Tag::ByteArray => NbtValue::ByteArray(NbtByteArray::nbt_iread(input)?),
		Tag::String => NbtValue::String(String::nbt_iread(input)?),
		Tag::List => NbtValue::List(NbtList::nbt_iread(input)?),
		Tag::Compound => NbtValue::Compound(NbtCompound::nbt_iread(input)?),
		Tag::IntArray => NbtValue::IntArray(NbtIntArray::nbt_iread(input)?),
		Tag::LongArray => NbtValue::LongArray(NbtLongArray::nbt_iread(input)?),
	};

	Ok(val)
}

pub fn read_seq<T: InternalNbtRead>(input: &mut &[u8]) -> Result<Vec<T>> {
	let size = i32::nbt_iread(input)?;

	if size.is_negative() {
		return Err(Error::InvalidLength(size));
	}

	let mut vec: Vec<T> = Vec::with_capacity(size as usize);

	match T::IS_POD {
		false => {
			for _ in 0..size {
				vec.push(T::nbt_iread(input)?);
			}
		}
		true => {
			let len_bytes = size as usize * T::StaticSize::USIZE;

			if input.len() < len_bytes {
				return Err(Error::NotEnoughData(len_bytes - input.len()));
			}

			unsafe {
				copy_nonoverlapping(input.as_ptr(), vec.as_mut_ptr() as *mut u8, len_bytes);
				vec.set_len(size as usize);

				// just need to swap endianness now

				// todo might want to do simd here btw
				let slice = slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, len_bytes);
				for chunk in slice.chunks_mut(T::StaticSize::USIZE) {
					#[cfg(target_endian = "little")]
					chunk.reverse();
				}
			}
			advance(input, len_bytes);
		}
	}

	Ok(vec)
}

/// mm yea... 🤤
fn advance(input: &mut &[u8], n: usize) {
	*input = &std::mem::take(input)[n..];
}

macro_rules! gen_impl_simple {
    ($($type:ty),*) => {$(
        impl InternalNbtRead for $type {
        	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
        		const SIZE: usize = size_of::<$type>();

				if input.len() < SIZE {
					return Err(Error::NotEnoughData(SIZE - input.len()));
				}

				let mut bytes = [0u8; SIZE];
				bytes.copy_from_slice(&input[..SIZE]);
				advance(input, SIZE);

				Ok(Self::from_be_bytes(bytes))
        	}
        }
    )*};
}
gen_impl_simple!(i8, i16, i32, i64, f32, f64);

impl InternalNbtRead for String {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
		if input.len() < 2 {
			return Err(Error::NotEnoughData(2 - input.len()));
		}
		let mut bytes = [0u8; 2];
		bytes.copy_from_slice(&input[..2]);
		advance(input, 2);
		let size = u16::from_be_bytes(bytes);

		if input.len() < size as usize {
			return Err(Error::NotEnoughData(size as usize - input.len()));
		}

		let decoded = simd_cesu8::decode_strict(&input[..size as usize])?;
		advance(input, size as usize);

		Ok(decoded.into_owned())
	}
}

impl InternalNbtRead for NbtByteArray {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
		Ok(Self(read_seq(input)?))
	}
}
impl InternalNbtRead for NbtIntArray {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
		Ok(Self(read_seq(input)?))
	}
}
impl InternalNbtRead for NbtLongArray {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
		Ok(Self(read_seq(input)?))
	}
}
impl InternalNbtRead for NbtList {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
		let v = match read_tag(input)? {
			Tag::End => {
				// only allowed if length is 0
				if i32::nbt_iread(input)? == 0 {
					return Ok(NbtList::Byte(Vec::new()));
				}

				return Err(Error::InvalidTag(0));
			}
			Tag::Byte => NbtList::Byte(read_seq(input)?),
			Tag::Short => NbtList::Short(read_seq(input)?),
			Tag::Int => NbtList::Int(read_seq(input)?),
			Tag::Long => NbtList::Long(read_seq(input)?),
			Tag::Float => NbtList::Float(read_seq(input)?),
			Tag::Double => NbtList::Double(read_seq(input)?),
			Tag::ByteArray => NbtList::ByteArray(read_seq(input)?),
			Tag::String => NbtList::String(read_seq(input)?),
			Tag::List => NbtList::List(read_seq(input)?),
			Tag::Compound => NbtList::Compound(read_seq(input)?),
			Tag::IntArray => NbtList::IntArray(read_seq(input)?),
			Tag::LongArray => NbtList::LongArray(read_seq(input)?),
		};

		Ok(v)
	}
}

impl InternalNbtRead for NbtCompound {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
		let mut map = NbtCompound::new();

		loop {
			let tag = read_tag(input)?;
			if tag == Tag::End {
				break;
			}

			let key = String::nbt_iread(input)?;
			let value = read_value(input, tag)?;

			match map.entry(key) {
				Entry::Occupied(entry) => {
					return Err(Error::KeyCollision(entry.key().clone()));
				}
				Entry::Vacant(entry) => {
					entry.insert(value);
				}
			}
		}

		Ok(map)
	}
}

impl<T: InternalNbtRead> InternalNbtRead for Vec<T> {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
		Ok(match read_tag(input)? {
			Tag::End => {
				// only allowed if length is 0
				if i32::nbt_iread(input)? != 0 {
					return Err(Error::InvalidTag(0));
				}

				Vec::new()
			}
			tag if tag == T::TAG => read_seq(input)?,
			other => {
				return Err(Error::WrongTag {
					field_name: "",
					expected: T::TAG,
					found: other,
				});
			}
		})
	}
}
impl<T: InternalNbtRead> InternalNbtRead for HashMap<String, T> {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self> {
		let mut map = HashMap::new();

		loop {
			let tag = read_tag(input)?;
			if tag == Tag::End {
				break;
			}
			if tag != T::TAG {
				return Err(Error::WrongTag {
					field_name: "",
					expected: T::TAG,
					found: tag,
				});
			}

			let key = String::nbt_iread(input)?;
			let value = T::nbt_iread(input)?;

			match map.entry(key) {
				Entry::Occupied(entry) => {
					return Err(Error::KeyCollision(entry.key().clone()));
				}
				Entry::Vacant(entry) => {
					entry.insert(value);
				}
			}
		}

		Ok(map)
	}
}
