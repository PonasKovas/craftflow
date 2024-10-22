use serde::{de::VariantAccess, Deserialize, Serialize};
use std::collections::HashMap;

use crate::arrays::{MAGIC_BYTE_ARRAY, MAGIC_INT_ARRAY, MAGIC_LONG_ARRAY};

/// A structure that can be used to represent any NBT tag dynamically
#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum DynNBT {
	Long(i64),
	Int(i32),
	Short(i16),
	Byte(i8),
	Double(f64),
	Float(f32),
	String(String),
	List(Vec<DynNBT>),
	Compound(HashMap<String, DynNBT>),
	LongArray(#[serde(with = "crate::arrays::long_array")] Vec<i64>),
	IntArray(#[serde(with = "crate::arrays::int_array")] Vec<i32>),
	ByteArray(#[serde(with = "crate::arrays::byte_array")] Vec<u8>),
}

// Due to the way serde derives Deserialize for untagged enums we need to implement it manually
// This is because the derived implementation does some weird type converting stuff, like deserializing
// all integers that fit in the range of 0-255 as u8, if the Deserialize wants an u8. This results in all
// integers being deserialized as Long (because they can all be converted to i64) if Long is defined first
// and if for example Byte is defined first, it will deserialize all integers in the range 0-255 as i8 regardless
// of their original type.
impl<'de> Deserialize<'de> for DynNBT {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		use serde::de::Error;
		struct DynNBTVisitor;

		impl<'de> serde::de::Visitor<'de> for DynNBTVisitor {
			type Value = DynNBT;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a valid NBT value")
			}
			fn visit_i8<E: Error>(self, v: i8) -> Result<Self::Value, E> {
				Ok(DynNBT::Byte(v))
			}
			fn visit_i16<E: Error>(self, v: i16) -> Result<Self::Value, E> {
				Ok(DynNBT::Short(v))
			}
			fn visit_i32<E: Error>(self, v: i32) -> Result<Self::Value, E> {
				Ok(DynNBT::Int(v))
			}
			fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
				Ok(DynNBT::Long(v))
			}
			fn visit_f32<E: Error>(self, v: f32) -> Result<Self::Value, E> {
				Ok(DynNBT::Float(v))
			}
			fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
				Ok(DynNBT::Double(v))
			}
			fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
				Ok(DynNBT::String(v))
			}
			fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
				Ok(DynNBT::String(v.to_owned()))
			}
			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let mut vec = Vec::new();
				while let Some(elem) = seq.next_element()? {
					vec.push(elem);
				}

				Ok(DynNBT::List(vec))
			}
			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::MapAccess<'de>,
			{
				let mut hash_map = HashMap::new();
				while let Some((key, value)) = map.next_entry()? {
					hash_map.insert(key, value);
				}
				Ok(DynNBT::Compound(hash_map))
			}
			fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::EnumAccess<'de>,
			{
				let (name, inner): (&str, _) = data.variant()?;

				match name {
					MAGIC_BYTE_ARRAY => {
						let array = inner.newtype_variant()?;
						Ok(DynNBT::ByteArray(array))
					}
					MAGIC_INT_ARRAY => {
						let array = inner.newtype_variant()?;
						Ok(DynNBT::IntArray(array))
					}
					MAGIC_LONG_ARRAY => {
						let array = inner.newtype_variant()?;
						Ok(DynNBT::LongArray(array))
					}
					_ => Err(A::Error::custom("Invalid magic byte array")),
				}
			}
		}

		deserializer.deserialize_any(DynNBTVisitor)
	}
}

#[cfg(test)]
mod tests {
	use super::DynNBT;
	use crate::{from_slice, tests::display_byte_buffer, to_writer};

	#[test]
	fn test_dyn_nbt() {
		let mut buffer = Vec::new();
		let value = DynNBT::Compound(
			vec![
				("byte".to_string(), DynNBT::Byte(42)),
				("short".to_string(), DynNBT::Short(42)),
				("int".to_string(), DynNBT::Int(42)),
				("long".to_string(), DynNBT::Long(42)),
				("float".to_string(), DynNBT::Float(42.0)),
				("double".to_string(), DynNBT::Double(42.0)),
				("byte_array".to_string(), DynNBT::ByteArray(vec![42])),
				("string".to_string(), DynNBT::String("42".to_string())),
				("list".to_string(), DynNBT::List(vec![DynNBT::Short(42)])),
				(
					"compound".to_string(),
					DynNBT::Compound(
						vec![("byte".to_string(), DynNBT::Byte(42))]
							.into_iter()
							.collect(),
					),
				),
				("int_array".to_string(), DynNBT::IntArray(vec![42])),
				("long_array".to_string(), DynNBT::LongArray(vec![42])),
			]
			.into_iter()
			.collect(),
		);
		to_writer(&mut buffer, &value).unwrap();
		let reconstructed: DynNBT = from_slice(&buffer).unwrap();

		if value != reconstructed {
			println!("Reconstructed: {:#?}", reconstructed);
			println!("Expected: {:#?}", value);
			println!("bytes: [{}]", display_byte_buffer(&buffer));
			println!("bytes: {:02x?}", &buffer);

			panic!("The reconstructed value does not match the original value!");
		}
	}
}
