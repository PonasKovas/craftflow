mod dyn_macro;

use crate::arrays::{MAGIC_BYTE_ARRAY, MAGIC_INT_ARRAY, MAGIC_LONG_ARRAY};
use serde::{de::VariantAccess, Deserialize, Serialize};
use shallowclone::ShallowClone;
use std::{
	borrow::Cow,
	collections::HashMap,
	ops::{Deref, DerefMut},
};

/// A structure that can be used to represent any NBT tag dynamically
#[derive(ShallowClone, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum DynNBT<'a> {
	Long(i64),
	Int(i32),
	Short(i16),
	Byte(i8),
	Double(f64),
	Float(f32),
	String(#[serde(borrow)] Cow<'a, str>),
	List(#[serde(borrow)] DynNBTList<'a>),
	Compound(#[serde(borrow)] DynNBTCompound<'a>),
	LongArray(
		#[serde(with = "crate::arrays::long_array")]
		#[serde(borrow)]
		Cow<'a, [i64]>,
	),
	IntArray(
		#[serde(with = "crate::arrays::int_array")]
		#[serde(borrow)]
		Cow<'a, [i32]>,
	),
	ByteArray(
		#[serde(with = "crate::arrays::byte_array")]
		#[serde(borrow)]
		Cow<'a, [u8]>,
	),
}

#[derive(ShallowClone, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
#[shallowclone(cow)]
pub enum DynNBTList<'a> {
	#[shallowclone(owned)]
	Owned(#[serde(borrow)] Vec<DynNBT<'a>>),
	#[serde(skip_deserializing)]
	#[shallowclone(borrowed)]
	Borrowed(&'a [DynNBT<'a>]),
}
impl<'a> From<Vec<DynNBT<'a>>> for DynNBTList<'a> {
	fn from(v: Vec<DynNBT<'a>>) -> Self {
		DynNBTList::Owned(v)
	}
}
impl<'a> From<&'a [DynNBT<'a>]> for DynNBTList<'a> {
	fn from(v: &'a [DynNBT<'a>]) -> Self {
		DynNBTList::Borrowed(v)
	}
}
impl<'a> Deref for DynNBTList<'a> {
	type Target = [DynNBT<'a>];

	fn deref(&self) -> &Self::Target {
		match self {
			DynNBTList::Owned(t) => t,
			DynNBTList::Borrowed(t) => t,
		}
	}
}
impl<'a> DerefMut for DynNBTList<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			DynNBTList::Owned(t) => t,
			DynNBTList::Borrowed(t) => {
				*self = DynNBTList::Owned(t.to_owned());
				match self {
					DynNBTList::Owned(t) => t,
					DynNBTList::Borrowed(_) => unreachable!(),
				}
			}
		}
	}
}

#[derive(ShallowClone, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
#[shallowclone(cow)]
pub enum DynNBTCompound<'a> {
	#[shallowclone(owned)]
	Owned(#[serde(borrow)] HashMap<Cow<'a, str>, DynNBT<'a>>),
	#[serde(skip_deserializing)]
	#[shallowclone(borrowed)]
	Borrowed(#[serde(borrow)] &'a HashMap<Cow<'a, str>, DynNBT<'a>>),
}
impl<'a> From<HashMap<Cow<'a, str>, DynNBT<'a>>> for DynNBTCompound<'a> {
	fn from(v: HashMap<Cow<'a, str>, DynNBT<'a>>) -> Self {
		DynNBTCompound::Owned(v)
	}
}
impl<'a> From<&'a HashMap<Cow<'a, str>, DynNBT<'a>>> for DynNBTCompound<'a> {
	fn from(v: &'a HashMap<Cow<'a, str>, DynNBT<'a>>) -> Self {
		DynNBTCompound::Borrowed(v)
	}
}
impl<'a> Deref for DynNBTCompound<'a> {
	type Target = HashMap<Cow<'a, str>, DynNBT<'a>>;

	fn deref(&self) -> &Self::Target {
		match self {
			DynNBTCompound::Owned(t) => t,
			DynNBTCompound::Borrowed(t) => t,
		}
	}
}
impl<'a> DerefMut for DynNBTCompound<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			DynNBTCompound::Owned(t) => t,
			DynNBTCompound::Borrowed(t) => {
				*self = DynNBTCompound::Owned(t.to_owned());
				match self {
					DynNBTCompound::Owned(t) => t,
					DynNBTCompound::Borrowed(_) => unreachable!(),
				}
			}
		}
	}
}

impl<'a> DynNBT<'a> {
	/// Recursively validates the NBT structure, making sure that all lists
	/// have elements only of the same type.
	pub fn validate(&self) -> Result<(), String> {
		match self {
			DynNBT::List(list) => {
				if list.is_empty() {
					return Ok(());
				}
				let first_tag = std::mem::discriminant(list.first().unwrap());
				for element in list.iter() {
					if std::mem::discriminant(element) != first_tag {
						return Err(format!("elements in list are not of the same type"));
					}
					element.validate()?;
				}
				Ok(())
			}
			DynNBT::Compound(compound) => {
				if compound.is_empty() {
					return Ok(());
				}
				for (name, element) in compound.iter() {
					match element.validate() {
						Ok(_) => {}
						Err(e) => return Err(format!("{name}: {e}")),
					}
				}
				Ok(())
			}
			_ => Ok(()),
		}
	}
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
	pub fn as_string(&self) -> Option<&Cow<'a, str>> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_list(&self) -> Option<&DynNBTList<'a>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_compound(&self) -> Option<&DynNBTCompound<'a>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_long_array(&self) -> Option<&Cow<'a, [i64]>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_int_array(&self) -> Option<&Cow<'a, [i32]>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_byte_array(&self) -> Option<&Cow<'a, [u8]>> {
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
	pub fn as_mut_string(&mut self) -> Option<&mut Cow<'a, str>> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_list(&mut self) -> Option<&mut DynNBTList<'a>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_compound(&mut self) -> Option<&mut DynNBTCompound<'a>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_long_array(&mut self) -> Option<&mut Cow<'a, [i64]>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_int_array(&mut self) -> Option<&mut Cow<'a, [i32]>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn as_mut_byte_array(&mut self) -> Option<&mut Cow<'a, [u8]>> {
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
	pub fn into_string(self) -> Option<Cow<'a, str>> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_list(self) -> Option<DynNBTList<'a>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_compound(self) -> Option<DynNBTCompound<'a>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_long_array(self) -> Option<Cow<'a, [i64]>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_int_array(self) -> Option<Cow<'a, [i32]>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	pub fn into_byte_array(self) -> Option<Cow<'a, [u8]>> {
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
	pub fn expect_string(&self) -> &Cow<'a, str> {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	pub fn expect_list(&self) -> &DynNBTList<'a> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	pub fn expect_compound(&self) -> &DynNBTCompound<'a> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	pub fn expect_long_array(&self) -> &Cow<'a, [i64]> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	pub fn expect_int_array(&self) -> &Cow<'a, [i32]> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	pub fn expect_byte_array(&self) -> &Cow<'a, [u8]> {
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
	pub fn expect_mut_string(&mut self) -> &mut Cow<'a, str> {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	pub fn expect_mut_list(&mut self) -> &mut DynNBTList<'a> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	pub fn expect_mut_compound(&mut self) -> &mut DynNBTCompound<'a> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	pub fn expect_mut_long_array(&mut self) -> &mut Cow<'a, [i64]> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	pub fn expect_mut_int_array(&mut self) -> &mut Cow<'a, [i32]> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	pub fn expect_mut_byte_array(&mut self) -> &mut Cow<'a, [u8]> {
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
	pub fn unwrap_string(self) -> Cow<'a, str> {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	pub fn unwrap_list(self) -> DynNBTList<'a> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	pub fn unwrap_compound(self) -> DynNBTCompound<'a> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	pub fn unwrap_long_array(self) -> Cow<'a, [i64]> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	pub fn unwrap_int_array(self) -> Cow<'a, [i32]> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	pub fn unwrap_byte_array(self) -> Cow<'a, [u8]> {
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
impl<'de> Deserialize<'de> for DynNBT<'de> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		use serde::de::Error;
		struct DynNBTVisitor;

		impl<'de> serde::de::Visitor<'de> for DynNBTVisitor {
			type Value = DynNBT<'de>;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("an NBT value")
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
				Ok(DynNBT::String(Cow::Owned(v)))
			}
			fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
				Ok(DynNBT::String(Cow::Owned(v.to_owned())))
			}
			fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> {
				Ok(DynNBT::String(Cow::Borrowed(v)))
			}
			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::SeqAccess<'de>,
			{
				let mut vec = Vec::new();
				while let Some(elem) = seq.next_element()? {
					vec.push(elem);
				}

				Ok(DynNBT::List(DynNBTList::Owned(vec)))
			}
			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::MapAccess<'de>,
			{
				let mut hash_map = HashMap::new();
				while let Some((key, value)) = map.next_entry()? {
					hash_map.insert(key, value);
				}
				Ok(DynNBT::Compound(DynNBTCompound::Owned(hash_map)))
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
			// The following methods are not used by the NBT deserializer
			// but we implement them regardless for general serde compatability
			////////////////////////////////////////////////////////////////////
			fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
				self.visit_i8(v as i8)
			}
			fn visit_u8<E: Error>(self, v: u8) -> Result<Self::Value, E> {
				self.visit_i8(v as i8)
			}
			fn visit_u16<E: Error>(self, v: u16) -> Result<Self::Value, E> {
				self.visit_i16(v as i16)
			}
			fn visit_u32<E: Error>(self, v: u32) -> Result<Self::Value, E> {
				self.visit_i32(v as i32)
			}
			fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
				self.visit_i64(v as i64)
			}
			fn visit_bytes<E: Error>(self, v: &[u8]) -> Result<Self::Value, E> {
				Ok(DynNBT::ByteArray(Cow::Owned(v.to_vec())))
			}
			fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> {
				Ok(DynNBT::ByteArray(Cow::Borrowed(v)))
			}
			fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
				Ok(DynNBT::ByteArray(Cow::Owned(v)))
			}
			fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				deserializer.deserialize_any(self)
			}
			fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				deserializer.deserialize_any(self)
			}
		}

		deserializer.deserialize_any(DynNBTVisitor)
	}
}

impl<'a> From<bool> for DynNBT<'a> {
	fn from(value: bool) -> Self {
		Self::Byte(value as i8)
	}
}
impl<'a> From<i8> for DynNBT<'a> {
	fn from(value: i8) -> Self {
		Self::Byte(value)
	}
}
impl<'a> From<u8> for DynNBT<'a> {
	fn from(value: u8) -> Self {
		Self::Byte(value as i8)
	}
}
impl<'a> From<i16> for DynNBT<'a> {
	fn from(value: i16) -> Self {
		Self::Short(value)
	}
}
impl<'a> From<u16> for DynNBT<'a> {
	fn from(value: u16) -> Self {
		Self::Short(value as i16)
	}
}
impl<'a> From<i32> for DynNBT<'a> {
	fn from(value: i32) -> Self {
		Self::Int(value)
	}
}
impl<'a> From<u32> for DynNBT<'a> {
	fn from(value: u32) -> Self {
		Self::Int(value as i32)
	}
}
impl<'a> From<i64> for DynNBT<'a> {
	fn from(value: i64) -> Self {
		Self::Long(value)
	}
}
impl<'a> From<u64> for DynNBT<'a> {
	fn from(value: u64) -> Self {
		Self::Long(value as i64)
	}
}
impl<'a> From<f32> for DynNBT<'a> {
	fn from(value: f32) -> Self {
		Self::Float(value)
	}
}
impl<'a> From<f64> for DynNBT<'a> {
	fn from(value: f64) -> Self {
		Self::Double(value)
	}
}
impl<'a> From<String> for DynNBT<'a> {
	fn from(value: String) -> Self {
		Self::String(Cow::Owned(value))
	}
}
impl<'a> From<&'a str> for DynNBT<'a> {
	fn from(value: &'a str) -> Self {
		Self::String(Cow::Borrowed(value))
	}
}
impl<'a, T: Into<DynNBT<'a>>> From<Vec<T>> for DynNBT<'a> {
	fn from(value: Vec<T>) -> Self {
		Self::List(DynNBTList::Owned(
			value.into_iter().map(Into::into).collect(),
		))
	}
}
impl<'a, T: Into<DynNBT<'a>>> From<HashMap<Cow<'a, str>, T>> for DynNBT<'a> {
	fn from(value: HashMap<Cow<'a, str>, T>) -> Self {
		Self::Compound(DynNBTCompound::Owned(
			value.into_iter().map(|(k, v)| (k, v.into())).collect(),
		))
	}
}
impl<'a, T: Into<DynNBT<'a>>> From<HashMap<String, T>> for DynNBT<'a> {
	fn from(value: HashMap<String, T>) -> Self {
		Self::Compound(DynNBTCompound::Owned(
			value
				.into_iter()
				.map(|(k, v)| (Cow::Owned(k), v.into()))
				.collect(),
		))
	}
}
impl<'a, T: Into<DynNBT<'a>>> From<HashMap<&'a str, T>> for DynNBT<'a> {
	fn from(value: HashMap<&'a str, T>) -> Self {
		Self::Compound(DynNBTCompound::Owned(
			value
				.into_iter()
				.map(|(k, v)| (Cow::Borrowed(k), v.into()))
				.collect(),
		))
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use super::DynNBT;
	use crate::{from_slice, tests::display_byte_buffer, to_writer};

	#[test]
	fn test_dyn_nbt() {
		let mut buffer = Vec::new();
		let value = DynNBT::Compound(
			vec![
				("byte".into(), DynNBT::Byte(42)),
				("short".into(), DynNBT::Short(42)),
				("int".into(), DynNBT::Int(42)),
				("long".into(), DynNBT::Long(42)),
				("float".into(), DynNBT::Float(42.0)),
				("double".into(), DynNBT::Double(42.0)),
				("byte_array".into(), DynNBT::ByteArray(vec![42].into())),
				("string".into(), DynNBT::String("42".into())),
				("list".into(), DynNBT::List(vec![DynNBT::Short(42)].into())),
				(
					"compound".into(),
					DynNBT::Compound(
						vec![("byte".into(), DynNBT::Byte(42))]
							.into_iter()
							.collect::<HashMap<_, _>>()
							.into(),
					),
				),
				("int_array".into(), DynNBT::IntArray(vec![42].into())),
				("long_array".into(), DynNBT::LongArray(vec![42].into())),
			]
			.into_iter()
			.collect::<HashMap<_, _>>()
			.into(),
		);
		to_writer(&mut buffer, &value).unwrap();
		let (input, reconstructed): (&[u8], DynNBT) = from_slice(&buffer).unwrap();
		assert!(input.is_empty());

		if value != reconstructed {
			println!("Reconstructed: {:#?}", reconstructed);
			println!("Expected: {:#?}", value);
			println!("bytes: [{}]", display_byte_buffer(&buffer));
			println!("bytes: {:02x?}", &buffer);

			panic!("The reconstructed value does not match the original value!");
		}
	}
}
