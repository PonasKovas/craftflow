use crate::{tag::Tag, Result};
use read_ext::ByteRead;
use serde::Deserialize;
use std::borrow::Cow;

pub(crate) mod any;
pub(crate) mod array_enum;
pub(crate) mod compound;
pub(crate) mod read_ext;
pub(crate) mod seq;

/// Deserializes any NBT structure from a byte slice
/// and returns the remaining slice together with the value
pub fn from_slice<'a, T: Deserialize<'a>>(mut input: &'a [u8]) -> Result<(&'a [u8], T)> {
	let mut deserializer = any::AnyDeserializer {
		input: &mut input,
		tag: None,
	};

	let r = T::deserialize(&mut deserializer)?;

	Ok((input, r))
}

/// Deserializes any NBT structure from a byte slice with a name
/// and returns the remaining slice together with the name and the value
pub fn from_slice_named<'a, T: Deserialize<'a>>(
	mut input: &'a [u8],
) -> Result<(&'a [u8], (Cow<'a, str>, T))> {
	let tag = Tag::new(input.read_u8()?)?;
	let name = input.read_str()?;

	let mut deserializer = any::AnyDeserializer {
		input: &mut input,
		tag: Some(tag),
	};

	let r = T::deserialize(&mut deserializer)?;

	Ok((input, (name, r)))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tag::Tag;

	#[test]
	fn test_deserialize() {
		fn inner_test<'a, T: Deserialize<'a> + PartialEq + std::fmt::Debug>(
			bytes: &'a [u8],
			expected_value: T,
		) {
			let result = from_slice::<'a, T>(bytes).unwrap();

			assert!(result.0.is_empty());
			assert_eq!(result.1, expected_value);
		}

		inner_test(&[Tag::End as u8], None::<()>);
		inner_test(&[Tag::Byte as u8, 1], Some(true));

		inner_test(&[Tag::Byte as u8, 1], true);
		inner_test(&[Tag::Byte as u8, 123], 123u8);
		inner_test(&[Tag::Short as u8, 0, 234], 234u16);
		inner_test(&[Tag::Int as u8, 0, 0, 0x1, 0x59], 345u32);
		inner_test(&[Tag::Long as u8, 0, 0, 0, 0, 0, 0, 0x1, 0xC8], 456u64);
		inner_test(&[Tag::Byte as u8, 100], 100i8);
		inner_test(&[Tag::Short as u8, 0x05, 0x28], 1320i16);
		inner_test(&[Tag::Int as u8, 0x00, 0x0F, 0x12, 0x06], 987654i32);
		#[rustfmt::skip]
		inner_test(
			&[
				Tag::List as u8,
				Tag::Short as u8,
				0, 0, 0, 3,
				0, 1,
				0, 2,
				0, 3,
			],
			vec![1u16, 2, 3],
		);

		#[derive(Deserialize, Debug, PartialEq)]
		struct Test1 {
			field: Vec<u16>,
		}
		#[rustfmt::skip]
		inner_test(
			&[
				Tag::Compound as u8,
				Tag::List as u8,
				0, 5,
				b'f', b'i', b'e', b'l', b'd',
				Tag::Short as u8,
				0, 0, 0, 1,
				0, 123,
				Tag::End as u8,
			],
			Test1 {
				field: vec![123u16],
			},
		);

		// more is tested doing data -> nbt -> data roundtrip tests
	}
	#[test]
	fn test_deserialize_named() {
		fn inner_test<'a, T: Deserialize<'a> + PartialEq + std::fmt::Debug>(
			bytes: &'a [u8],
			expected_name: &str,
			expected_value: T,
		) {
			let (input, (name, result)) = from_slice_named::<'a, T>(bytes).unwrap();

			assert!(input.is_empty());

			assert_eq!(name, expected_name);
			assert_eq!(result, expected_value);
		}

		inner_test(
			&[Tag::Byte as u8, 0, 4, b'h', b'e', b'l', b'o', 1],
			"helo",
			true,
		);
		inner_test(
			&[Tag::List as u8, 0, 0, Tag::Short as u8, 0, 0, 0, 1, 0, 123],
			"",
			vec![123u16],
		);
		// enough testing
	}
}
