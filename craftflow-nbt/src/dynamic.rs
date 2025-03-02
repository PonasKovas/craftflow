use crate::arrays::{MAGIC_BYTE_ARRAY, MAGIC_INT_ARRAY, MAGIC_LONG_ARRAY};
use serde::{de::VariantAccess, Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

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
	List(#[serde(borrow)] CoCowSlice<'a, DynNBT<'a>>),
	Compound(#[serde(borrow)] CoCow<'a, HashMap<Cow<'a, str>, DynNBT<'a>>>),
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
	/// Returns the value of any integer type (byte, short, int, long)
	pub fn as_int_nonstrict(&self) -> Option<i64> {
		match self {
			DynNBT::Byte(v) => Some(*v as i64),
			DynNBT::Short(v) => Some(*v as i64),
			DynNBT::Int(v) => Some(*v as i64),
			DynNBT::Long(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns the value of any integer type (byte, short, int, long)
	///
	/// This is the same as [`DynNBT::as_int_nonstrict()`], but it consumes the NBT.
	pub fn into_int_nonstrict(self) -> Option<i64> {
		self.as_int_nonstrict()
	}
	/// Returns the value of any integer type (byte, short, int, long), panicking if it's not an integer
	pub fn expect_int_nonstrict(&self) -> i64 {
		match self.as_int_nonstrict() {
			Some(v) => v,
			None => panic!("Expected integer, found {:?}", self),
		}
	}
	/// Returns the value of any integer type (byte, short, int, long), panicking if it's not an integer
	///
	/// This is the same as [`DynNBT::expect_int_nonstrict()`], but it consumes the NBT.
	pub fn unwrap_int_nonstrict(self) -> i64 {
		self.expect_int_nonstrict()
	}
	/// Returns a Long, if this NBT is a long.
	pub fn as_long(&self) -> Option<i64> {
		match self {
			DynNBT::Long(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns an Int, if this NBT is an int.
	pub fn as_int(&self) -> Option<i32> {
		match self {
			DynNBT::Int(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a Short, if this NBT is a short.
	pub fn as_short(&self) -> Option<i16> {
		match self {
			DynNBT::Short(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a Byte, if this NBT is a byte.
	pub fn as_byte(&self) -> Option<i8> {
		match self {
			DynNBT::Byte(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a Double, if this NBT is a double.
	pub fn as_double(&self) -> Option<f64> {
		match self {
			DynNBT::Double(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a Float, if this NBT is a float.
	pub fn as_float(&self) -> Option<f32> {
		match self {
			DynNBT::Float(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a String, if this NBT is a string.
	pub fn as_string(&self) -> Option<&Cow<'a, str>> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a List, if this NBT is a list.
	pub fn as_list(&self) -> Option<&CoCowSlice<'a, DynNBT<'a>>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a Compound, if this NBT is a compound.
	pub fn as_compound(&self) -> Option<&CoCow<'a, HashMap<Cow<'a, str>, DynNBT<'a>>>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a LongArray, if this NBT is a long array.
	pub fn as_long_array(&self) -> Option<&Cow<'a, [i64]>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a IntArray, if this NBT is an int array.
	pub fn as_int_array(&self) -> Option<&Cow<'a, [i32]>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a ByteArray, if this NBT is a byte array.
	pub fn as_byte_array(&self) -> Option<&Cow<'a, [u8]>> {
		match self {
			DynNBT::ByteArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a long.
	pub fn as_mut_long(&mut self) -> Option<&mut i64> {
		match self {
			DynNBT::Long(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is an int.
	pub fn as_mut_int(&mut self) -> Option<&mut i32> {
		match self {
			DynNBT::Int(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a short.
	pub fn as_mut_short(&mut self) -> Option<&mut i16> {
		match self {
			DynNBT::Short(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a byte.
	pub fn as_mut_byte(&mut self) -> Option<&mut i8> {
		match self {
			DynNBT::Byte(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a double.
	pub fn as_mut_double(&mut self) -> Option<&mut f64> {
		match self {
			DynNBT::Double(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a float.
	pub fn as_mut_float(&mut self) -> Option<&mut f32> {
		match self {
			DynNBT::Float(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a string.
	pub fn as_mut_string(&mut self) -> Option<&mut Cow<'a, str>> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a list.
	pub fn as_mut_list(&mut self) -> Option<&mut CoCowSlice<'a, DynNBT<'a>>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a compound.
	pub fn as_mut_compound(&mut self) -> Option<&mut CoCow<'a, HashMap<Cow<'a, str>, DynNBT<'a>>>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a long array.
	pub fn as_mut_long_array(&mut self) -> Option<&mut Cow<'a, [i64]>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is an int array.
	pub fn as_mut_int_array(&mut self) -> Option<&mut Cow<'a, [i32]>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a byte array.
	pub fn as_mut_byte_array(&mut self) -> Option<&mut Cow<'a, [u8]>> {
		match self {
			DynNBT::ByteArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a long, if this NBT is a long.
	///
	/// This is the same as [`DynNBT::as_long()`], but it consumes the NBT.
	pub fn into_long(self) -> Option<i64> {
		match self {
			DynNBT::Long(v) => Some(v),
			_ => None,
		}
	}
	/// Returns an int, if this NBT is an int.
	///
	/// This is the same as [`DynNBT::as_int()`], but it consumes the NBT.
	pub fn into_int(self) -> Option<i32> {
		match self {
			DynNBT::Int(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a short, if this NBT is a short.
	///
	/// This is the same as [`DynNBT::as_short()`], but it consumes the NBT.
	pub fn into_short(self) -> Option<i16> {
		match self {
			DynNBT::Short(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a byte, if this NBT is a byte.
	///
	/// This is the same as [`DynNBT::as_byte()`], but it consumes the NBT.
	pub fn into_byte(self) -> Option<i8> {
		match self {
			DynNBT::Byte(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a double, if this NBT is a double.
	///
	/// This is the same as [`DynNBT::as_double()`], but it consumes the NBT.
	pub fn into_double(self) -> Option<f64> {
		match self {
			DynNBT::Double(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a float, if this NBT is a float.
	///
	/// This is the same as [`DynNBT::as_float()`], but it consumes the NBT.
	pub fn into_float(self) -> Option<f32> {
		match self {
			DynNBT::Float(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a string by value, if this NBT is a string.
	pub fn into_string(self) -> Option<Cow<'a, str>> {
		match self {
			DynNBT::String(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a list by value, if this NBT is a list.
	pub fn into_list(self) -> Option<CoCowSlice<'a, DynNBT<'a>>> {
		match self {
			DynNBT::List(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a compound by value, if this NBT is a compound.
	pub fn into_compound(self) -> Option<CoCow<'a, HashMap<Cow<'a, str>, DynNBT<'a>>>> {
		match self {
			DynNBT::Compound(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a long array by value, if this NBT is a long array.
	pub fn into_long_array(self) -> Option<Cow<'a, [i64]>> {
		match self {
			DynNBT::LongArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns an int array by value, if this NBT is an int array.
	pub fn into_int_array(self) -> Option<Cow<'a, [i32]>> {
		match self {
			DynNBT::IntArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a byte array by value, if this NBT is a byte array.
	pub fn into_byte_array(self) -> Option<Cow<'a, [u8]>> {
		match self {
			DynNBT::ByteArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a long, if this NBT is a long, panics otherwise.
	pub fn expect_long(&self) -> i64 {
		match self {
			DynNBT::Long(v) => *v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	/// Returns an int, if this NBT is an int, panics otherwise.
	pub fn expect_int(&self) -> i32 {
		match self {
			DynNBT::Int(v) => *v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	/// Returns a short, if this NBT is a short, panics otherwise.
	pub fn expect_short(&self) -> i16 {
		match self {
			DynNBT::Short(v) => *v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	/// Returns a byte, if this NBT is a byte, panics otherwise.
	pub fn expect_byte(&self) -> i8 {
		match self {
			DynNBT::Byte(v) => *v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	/// Returns a double, if this NBT is a double, panics otherwise.
	pub fn expect_double(&self) -> f64 {
		match self {
			DynNBT::Double(v) => *v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	/// Returns a float, if this NBT is a float, panics otherwise.
	pub fn expect_float(&self) -> f32 {
		match self {
			DynNBT::Float(v) => *v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	/// Returns a string reference, if this NBT is a string, panics otherwise.
	pub fn expect_string(&self) -> &Cow<'a, str> {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	/// Returns a byte array reference, if this NBT is a byte array, panics otherwise.
	pub fn expect_list(&self) -> &CoCowSlice<'a, DynNBT<'a>> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	/// Returns a compound reference, if this NBT is a compound, panics otherwise.
	pub fn expect_compound(&self) -> &CoCow<'a, HashMap<Cow<'a, str>, DynNBT<'a>>> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	/// Returns a long array reference, if this NBT is a long array, panics otherwise.
	pub fn expect_long_array(&self) -> &Cow<'a, [i64]> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	/// Returns an int array reference, if this NBT is an int array, panics otherwise.
	pub fn expect_int_array(&self) -> &Cow<'a, [i32]> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	/// Returns a byte array reference, if this NBT is a byte array, panics otherwise.
	pub fn expect_byte_array(&self) -> &Cow<'a, [u8]> {
		match self {
			DynNBT::ByteArray(v) => v,
			_ => panic!("Expected ByteArray, found {:?}", self),
		}
	}
	/// Returns a long by mutable reference, if this NBT is a long, panics otherwise.
	pub fn expect_mut_long(&mut self) -> &mut i64 {
		match self {
			DynNBT::Long(v) => v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	/// Returns an int by mutable reference, if this NBT is an int, panics otherwise.
	pub fn expect_mut_int(&mut self) -> &mut i32 {
		match self {
			DynNBT::Int(v) => v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	/// Returns a short by mutable reference, if this NBT is a short, panics otherwise.
	pub fn expect_mut_short(&mut self) -> &mut i16 {
		match self {
			DynNBT::Short(v) => v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	/// Returns a byte by mutable reference, if this NBT is a byte, panics otherwise.
	pub fn expect_mut_byte(&mut self) -> &mut i8 {
		match self {
			DynNBT::Byte(v) => v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	/// Returns a double by mutable reference, if this NBT is a double, panics otherwise.
	pub fn expect_mut_double(&mut self) -> &mut f64 {
		match self {
			DynNBT::Double(v) => v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	/// Returns a float by mutable reference, if this NBT is a float, panics otherwise.
	pub fn expect_mut_float(&mut self) -> &mut f32 {
		match self {
			DynNBT::Float(v) => v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	/// Returns a string by mutable reference, if this NBT is a string, panics otherwise.
	pub fn expect_mut_string(&mut self) -> &mut Cow<'a, str> {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	/// Returns a byte array by mutable reference, if this NBT is a list, panics otherwise.
	pub fn expect_mut_list(&mut self) -> &mut CoCowSlice<'a, DynNBT<'a>> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	/// Returns a compound by mutable reference, if this NBT is a compound, panics otherwise.
	pub fn expect_mut_compound(&mut self) -> &mut CoCow<'a, HashMap<Cow<'a, str>, DynNBT<'a>>> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	/// Returns a long array by mutable reference, if this NBT is a long array, panics otherwise.
	pub fn expect_mut_long_array(&mut self) -> &mut Cow<'a, [i64]> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	/// Returns an int array by mutable reference, if this NBT is an int array, panics otherwise.
	pub fn expect_mut_int_array(&mut self) -> &mut Cow<'a, [i32]> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	/// Returns a byte array by mutable reference, if this NBT is a byte array, panics otherwise.
	pub fn expect_mut_byte_array(&mut self) -> &mut Cow<'a, [u8]> {
		match self {
			DynNBT::ByteArray(v) => v,
			_ => panic!("Expected ByteArray, found {:?}", self),
		}
	}
	/// Returns a long, if this NBT is a long, panics otherwise.
	///
	/// This is the same as [`DynNBT::expect_long()`], but it consumes the NBT.
	pub fn unwrap_long(self) -> i64 {
		match self {
			DynNBT::Long(v) => v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	/// Returns an int, if this NBT is an int, panics otherwise.
	///
	/// This is the same as [`DynNBT::expect_int()`], but it consumes the NBT.
	pub fn unwrap_int(self) -> i32 {
		match self {
			DynNBT::Int(v) => v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	/// Returns a short, if this NBT is a short, panics otherwise.
	///
	/// This is the same as [`DynNBT::expect_short()`], but it consumes the NBT.
	pub fn unwrap_short(self) -> i16 {
		match self {
			DynNBT::Short(v) => v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	/// Returns a byte, if this NBT is a byte, panics otherwise.
	///
	/// This is the same as [`DynNBT::expect_byte()`], but it consumes the NBT.
	pub fn unwrap_byte(self) -> i8 {
		match self {
			DynNBT::Byte(v) => v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	/// Returns a double, if this NBT is a double, panics otherwise.
	///
	/// This is the same as [`DynNBT::expect_double()`], but it consumes the NBT.
	pub fn unwrap_double(self) -> f64 {
		match self {
			DynNBT::Double(v) => v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	/// Returns a float, if this NBT is a float, panics otherwise.
	///
	/// This is the same as [`DynNBT::expect_float()`], but it consumes the NBT.
	pub fn unwrap_float(self) -> f32 {
		match self {
			DynNBT::Float(v) => v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	/// Returns a string by value, if this NBT is a string, panics otherwise.
	pub fn unwrap_string(self) -> Cow<'a, str> {
		match self {
			DynNBT::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	/// Returns a list by value, if this NBT is a list, panics otherwise.
	pub fn unwrap_list(self) -> CoCowSlice<'a, DynNBT<'a>> {
		match self {
			DynNBT::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	/// Returns a compound by value, if this NBT is a compound, panics otherwise.
	pub fn unwrap_compound(self) -> CoCow<'a, HashMap<Cow<'a, str>, DynNBT<'a>>> {
		match self {
			DynNBT::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	/// Returns a long array by value, if this NBT is a long array, panics otherwise.
	pub fn unwrap_long_array(self) -> Cow<'a, [i64]> {
		match self {
			DynNBT::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	/// Returns an int array by value, if this NBT is an int array, panics otherwise.
	pub fn unwrap_int_array(self) -> Cow<'a, [i32]> {
		match self {
			DynNBT::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	/// Returns a byte array by value, if this NBT is a byte array, panics otherwise.
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

				Ok(DynNBT::List(CoCowSlice::Owned(vec)))
			}
			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: serde::de::MapAccess<'de>,
			{
				let mut hash_map = HashMap::new();
				while let Some((key, value)) = map.next_entry()? {
					hash_map.insert(key, value);
				}
				Ok(DynNBT::Compound(CoCow::Owned(hash_map)))
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
impl<'a> From<Cow<'a, str>> for DynNBT<'a> {
	fn from(value: Cow<'a, str>) -> Self {
		Self::String(value)
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
		Self::List(CoCowSlice::Owned(
			value.into_iter().map(Into::into).collect(),
		))
	}
}
impl<'a, T: Into<DynNBT<'a>>> From<HashMap<Cow<'a, str>, T>> for DynNBT<'a> {
	fn from(value: HashMap<Cow<'a, str>, T>) -> Self {
		Self::Compound(CoCow::Owned(
			value.into_iter().map(|(k, v)| (k, v.into())).collect(),
		))
	}
}
impl<'a, T: Into<DynNBT<'a>>> From<HashMap<String, T>> for DynNBT<'a> {
	fn from(value: HashMap<String, T>) -> Self {
		Self::Compound(CoCow::Owned(
			value
				.into_iter()
				.map(|(k, v)| (Cow::Owned(k), v.into()))
				.collect(),
		))
	}
}
impl<'a, T: Into<DynNBT<'a>>> From<HashMap<&'a str, T>> for DynNBT<'a> {
	fn from(value: HashMap<&'a str, T>) -> Self {
		Self::Compound(CoCow::Owned(
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
