use crate::Error;
use any::AnySerializer;
use compound::CompoundSerializer;
use serde::{ser::SerializeMap, Serialize};
use std::io::Write;

pub(crate) mod any;
pub(crate) mod compound;
pub(crate) mod compound_key;
pub(crate) mod seq;
pub(crate) mod tag;
pub(crate) mod write_str;

/// Serializes any value in the NBT format and returns the number of bytes written.
pub fn to_writer<W: Write, S>(mut writer: W, value: &S) -> Result<usize, Error>
where
	S: Serialize,
{
	let serializer = AnySerializer {
		output: &mut writer,
		expecting: None,
	};
	value.serialize(serializer)
}

/// Serializes any value with a name in the NBT format and returns the number of bytes written.
pub fn to_writer_named<W: Write, S>(mut writer: W, name: &str, value: &S) -> Result<usize, Error>
where
	S: Serialize,
{
	let mut serializer = CompoundSerializer::new(&mut writer, 0);
	serializer.serialize_entry(name, value)?;
	// return without ending (we dont need the extra TAG_END, because the compound is implicit,
	// just like we didnt add TAG_COMPOUND at the beginning)
	Ok(serializer.written)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{tag::Tag, tests::display_byte_buffer};

	#[test]
	fn test_serialize() {
		fn inner_test<T: Serialize + std::fmt::Debug>(value: T, expected_bytes: Vec<u8>) {
			let mut buffer = Vec::new();
			let bytes_written = to_writer(&mut buffer, &value).unwrap();

			assert_eq!(
				bytes_written,
				buffer.len(),
				"[{}]\n\n[{}]",
				display_byte_buffer(&buffer),
				display_byte_buffer(&expected_bytes)
			);
			assert_eq!(buffer, expected_bytes, "{value:?}");
		}

		inner_test(true, vec![Tag::Byte as u8, 1]);
		inner_test(123u8, vec![Tag::Byte as u8, 123]);
		inner_test(234u16, vec![Tag::Short as u8, 0, 234]);
		inner_test(345u32, vec![Tag::Int as u8, 0, 0, 0x1, 0x59]);
		inner_test(456u64, vec![Tag::Long as u8, 0, 0, 0, 0, 0, 0, 0x1, 0xC8]);
		inner_test(100i8, vec![Tag::Byte as u8, 100]);
		inner_test(1320i16, vec![Tag::Short as u8, 0x05, 0x28]);
		inner_test(987654i32, vec![Tag::Int as u8, 0x00, 0x0F, 0x12, 0x06]);
		inner_test(
			159003i64,
			vec![Tag::Long as u8, 0, 0, 0, 0, 0, 0x02, 0x6D, 0x1B],
		);

		let mut b32 = vec![Tag::Float as u8];
		b32.extend_from_slice(&3.14f32.to_be_bytes());
		inner_test(3.14f32, b32);

		let mut b64 = vec![Tag::Double as u8];
		b64.extend_from_slice(&3.14f64.to_be_bytes());
		inner_test(3.14f64, b64);

		inner_test(
			&[1u8, 2, 3, 4],
			vec![Tag::List as u8, Tag::Byte as u8, 0, 0, 0, 4, 1, 2, 3, 4],
		);
		#[rustfmt::skip]
		inner_test(
			&vec![vec![1i8, 2], vec![3, 4], vec![5, 6]],
			vec![
				Tag::List as u8,
				Tag::List as u8,
				0, 0, 0, 3,
				Tag::Byte as u8,
				0, 0, 0, 2,
				1,
				2,
				Tag::Byte as u8,
				0, 0, 0, 2,
				3,
				4,
				Tag::Byte as u8,
				0, 0, 0, 2,
				5,
				6,
			],
		);

		#[rustfmt::skip]
		inner_test(
			"hi \0 :)",
			vec![Tag::String as u8, 0, 8, b'h', b'i', b' ', 0xC0, 0x80, b' ', b':', b')'],
		);

		#[derive(Serialize, Debug)]
		struct Compound {
			test: bool,
			test2: Inner,
		}
		#[derive(Serialize, Debug)]
		struct Inner {
			test3: i16,
		}

		#[rustfmt::skip]
		inner_test(
			&Compound { test: true, test2: Inner { test3: -3 } },
			vec![Tag::Compound as u8,
			        Tag::Byte as u8,
			        0, 4,
			        b't', b'e', b's', b't',
			        true as u8,

			        Tag::Compound as u8,
   					0, 5,
 			        b't', b'e', b's', b't', b'2',
    			        Tag::Short as u8,
    			        0, 5,
    			        b't', b'e', b's', b't', b'3',
    			        0xFF, 0xFD,
			        0,
				0,],
		);

		#[rustfmt::skip]
		inner_test::<&[()]>(
			&[],
			vec![Tag::List as u8, Tag::End as u8, 0, 0, 0, 0],
		);

		inner_test::<Option<()>>(None, vec![Tag::End as u8]);

		#[derive(Serialize, Debug)]
		struct OptionTest {
			test: Option<bool>,
		}

		inner_test(
			OptionTest { test: None },
			vec![Tag::Compound as u8, Tag::End as u8],
		);
	}

	#[test]
	fn test_named_serialize() {
		let mut buffer = Vec::new();
		let bytes_written = to_writer_named(&mut buffer, "root", &123u16).unwrap();

		assert_eq!(bytes_written, buffer.len(), "written bytes doesnt match");
		#[rustfmt::skip]
		assert_eq!(
			buffer,
			vec![
			    Tag::Short as u8,
				0x00, 0x04,
				b'r', b'o', b'o', b't',
		        0, 123
			]
		);
	}
}
