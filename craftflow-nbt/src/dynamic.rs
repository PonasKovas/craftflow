use crate::arrays::{MAGIC_BYTE_ARRAY, MAGIC_INT_ARRAY, MAGIC_LONG_ARRAY};
use serde::{de::VariantAccess, Deserialize, Serialize};
use std::collections::HashMap;

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

impl DynNBT {
	pub fn as_long(&self) -> Option<i64> {
		match self {
			DynNBT::Long(v) => Some(*v),
			_ => None,
		}
	}
	pub fn as_int(&self) -> Option<i32> {
		match self {
			DynNBT::Int(v) => Some(*v),
			_ => None,
		}
	}
	pub fn as_short(&self) -> Option<i16> {
		match self {
			DynNBT::Short(v) => Some(*v),
			_ => None,
		}
	}
	pub fn as_byte(&self) -> Option<i8> {
		match self {
			DynNBT::Byte(v) => Some(*v),
			_ => None,
		}
	}
	pub fn as_double(&self) -> Option<f64> {
		match self {
			DynNBT::Double(v) => Some(*v),
			_ => None,
		}
	}
	pub fn as_float(&self) -> Option<f32> {
		match self {
			DynNBT::Float(v) => Some(*v),
			_ => None,
		}
	}
	pub fn as_string(&self) -> Option<&String> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_list(&self) -> Option<&Vec<DynNBT>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_compound(&self) -> Option<&HashMap<String, DynNBT>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_long_array(&self) -> Option<&Vec<i64>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_int_array(&self) -> Option<&Vec<i32>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_byte_array(&self) -> Option<&Vec<u8>> {
		match self {
			DynNBT::ByteArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_long(&mut self) -> Option<&mut i64> {
		match self {
			DynNBT::Long(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_int(&mut self) -> Option<&mut i32> {
		match self {
			DynNBT::Int(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_short(&mut self) -> Option<&mut i16> {
		match self {
			DynNBT::Short(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_byte(&mut self) -> Option<&mut i8> {
		match self {
			DynNBT::Byte(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_double(&mut self) -> Option<&mut f64> {
		match self {
			DynNBT::Double(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_float(&mut self) -> Option<&mut f32> {
		match self {
			DynNBT::Float(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_string(&mut self) -> Option<&mut String> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_list(&mut self) -> Option<&mut Vec<DynNBT>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_compound(&mut self) -> Option<&mut HashMap<String, DynNBT>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_long_array(&mut self) -> Option<&mut Vec<i64>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_int_array(&mut self) -> Option<&mut Vec<i32>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_byte_array(&mut self) -> Option<&mut Vec<u8>> {
		match self {
			DynNBT::ByteArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_long(self) -> Option<i64> {
		match self {
			DynNBT::Long(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_int(self) -> Option<i32> {
		match self {
			DynNBT::Int(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_short(self) -> Option<i16> {
		match self {
			DynNBT::Short(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_byte(self) -> Option<i8> {
		match self {
			DynNBT::Byte(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_double(self) -> Option<f64> {
		match self {
			DynNBT::Double(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_float(self) -> Option<f32> {
		match self {
			DynNBT::Float(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_string(self) -> Option<String> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_list(self) -> Option<Vec<DynNBT>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_compound(self) -> Option<HashMap<String, DynNBT>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_long_array(self) -> Option<Vec<i64>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_int_array(self) -> Option<Vec<i32>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_byte_array(self) -> Option<Vec<u8>> {
		match self {
			DynNBT::ByteArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn expect_long(&self) -> i64 {
		match self {
			DynNBT::Long(v) => *v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	pub fn expect_int(&self) -> i32 {
		match self {
			DynNBT::Int(v) => *v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	pub fn expect_short(&self) -> i16 {
		match self {
			DynNBT::Short(v) => *v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	pub fn expect_byte(&self) -> i8 {
		match self {
			DynNBT::Byte(v) => *v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	pub fn expect_double(&self) -> f64 {
		match self {
			DynNBT::Double(v) => *v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	pub fn expect_float(&self) -> f32 {
		match self {
			DynNBT::Float(v) => *v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	pub fn expect_string(&self) -> &String {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	pub fn expect_list(&self) -> &Vec<DynNBT> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	pub fn expect_compound(&self) -> &HashMap<String, DynNBT> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	pub fn expect_long_array(&self) -> &Vec<i64> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	pub fn expect_int_array(&self) -> &Vec<i32> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	pub fn expect_byte_array(&self) -> &Vec<u8> {
		match self {
			DynNBT::ByteArray(v) => v,
			_ => panic!("Expected ByteArray, found {:?}", self),
		}
	}
	pub fn expect_mut_long(&mut self) -> &mut i64 {
		match self {
			DynNBT::Long(v) => v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	pub fn expect_mut_int(&mut self) -> &mut i32 {
		match self {
			DynNBT::Int(v) => v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	pub fn expect_mut_short(&mut self) -> &mut i16 {
		match self {
			DynNBT::Short(v) => v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	pub fn expect_mut_byte(&mut self) -> &mut i8 {
		match self {
			DynNBT::Byte(v) => v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	pub fn expect_mut_double(&mut self) -> &mut f64 {
		match self {
			DynNBT::Double(v) => v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	pub fn expect_mut_float(&mut self) -> &mut f32 {
		match self {
			DynNBT::Float(v) => v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	pub fn expect_mut_string(&mut self) -> &mut String {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	pub fn expect_mut_list(&mut self) -> &mut Vec<DynNBT> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	pub fn expect_mut_compound(&mut self) -> &mut HashMap<String, DynNBT> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	pub fn expect_mut_long_array(&mut self) -> &mut Vec<i64> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	pub fn expect_mut_int_array(&mut self) -> &mut Vec<i32> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	pub fn expect_mut_byte_array(&mut self) -> &mut Vec<u8> {
		match self {
			DynNBT::ByteArray(v) => v,
			_ => panic!("Expected ByteArray, found {:?}", self),
		}
	}
	pub fn unwrap_long(self) -> i64 {
		match self {
			DynNBT::Long(v) => v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	pub fn unwrap_int(self) -> i32 {
		match self {
			DynNBT::Int(v) => v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	pub fn unwrap_short(self) -> i16 {
		match self {
			DynNBT::Short(v) => v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	pub fn unwrap_byte(self) -> i8 {
		match self {
			DynNBT::Byte(v) => v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	pub fn unwrap_double(self) -> f64 {
		match self {
			DynNBT::Double(v) => v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	pub fn unwrap_float(self) -> f32 {
		match self {
			DynNBT::Float(v) => v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	pub fn unwrap_string(self) -> String {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	pub fn unwrap_list(self) -> Vec<DynNBT> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	pub fn unwrap_compound(self) -> HashMap<String, DynNBT> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	pub fn unwrap_long_array(self) -> Vec<i64> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	pub fn unwrap_int_array(self) -> Vec<i32> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	pub fn unwrap_byte_array(self) -> Vec<u8> {
		match self {
			DynNBT::ByteArray(v) => v,
			_ => panic!("Expected ByteArray, found {:?}", self),
		}
	}
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
