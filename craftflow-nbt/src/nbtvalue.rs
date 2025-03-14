use crate::{
	NbtRead, NbtStr, NbtString, NbtWrite,
	internal::{
		InternalNbtRead, InternalNbtWrite,
		read::{read_tag, read_value},
		write::{write_tag, write_value},
	},
	tag::Tag,
};
use std::{
	collections::HashMap,
	ops::{Deref, DerefMut},
};

/// NBT Compound type - essentially a map
pub type NbtCompound = HashMap<NbtString, NbtValue>;

/// Any Nbt value
#[derive(Debug, Clone, PartialEq)]
pub enum NbtValue {
	Byte(i8),
	Short(i16),
	Int(i32),
	Long(i64),
	Float(f32),
	Double(f64),
	ByteArray(NbtByteArray),
	String(NbtString),
	List(NbtList),
	Compound(NbtCompound),
	IntArray(NbtIntArray),
	LongArray(NbtLongArray),
}

impl NbtValue {
	pub(crate) fn tag(&self) -> Tag {
		match self {
			Self::Byte(_) => Tag::Byte,
			Self::Short(_) => Tag::Short,
			Self::Int(_) => Tag::Int,
			Self::Long(_) => Tag::Long,
			Self::Float(_) => Tag::Float,
			Self::Double(_) => Tag::Double,
			Self::ByteArray(_) => Tag::ByteArray,
			Self::String(_) => Tag::String,
			Self::List(_) => Tag::List,
			Self::Compound(_) => Tag::Compound,
			Self::IntArray(_) => Tag::IntArray,
			Self::LongArray(_) => Tag::LongArray,
		}
	}
}

// NBT value implements these directly without implement InternalNbtRead/Write
// Because its not an NBT primitive, for example you cant have a list of NbtValues
//
// This is just the top level user-facing API for dynamic values
impl NbtWrite for NbtValue {
	fn nbt_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(self.tag(), output);
		written += write_value(self, output);

		written
	}
	fn nbt_write_named(&self, name: &NbtStr, output: &mut Vec<u8>) -> usize {
		let mut written = 0;

		written += write_tag(self.tag(), output);
		written += name.nbt_iwrite(output);
		written += write_value(self, output);

		written
	}
}
impl NbtRead for NbtValue {
	fn nbt_read(input: &mut &[u8]) -> crate::Result<Self> {
		let tag = read_tag(input)?;

		read_value(input, tag)
	}
	fn nbt_read_named(input: &mut &[u8]) -> crate::Result<(NbtString, Self)> {
		let tag = read_tag(input)?;
		let name = NbtString::nbt_iread(input)?;

		read_value(input, tag).map(|v| (name, v))
	}
}

/// NBT list of values
#[derive(Debug, Clone, PartialEq)]
pub enum NbtList {
	Byte(Vec<i8>),
	Short(Vec<i16>),
	Int(Vec<i32>),
	Long(Vec<i64>),
	Float(Vec<f32>),
	Double(Vec<f64>),
	ByteArray(Vec<NbtByteArray>),
	String(Vec<NbtString>),
	List(Vec<NbtList>),
	Compound(Vec<NbtCompound>),
	IntArray(Vec<NbtIntArray>),
	LongArray(Vec<NbtLongArray>),
}

impl NbtList {
	pub(crate) fn tag(&self) -> Tag {
		match self {
			Self::Byte(_) => Tag::Byte,
			Self::Short(_) => Tag::Short,
			Self::Int(_) => Tag::Int,
			Self::Long(_) => Tag::Long,
			Self::Float(_) => Tag::Float,
			Self::Double(_) => Tag::Double,
			Self::ByteArray(_) => Tag::ByteArray,
			Self::String(_) => Tag::String,
			Self::List(_) => Tag::List,
			Self::Compound(_) => Tag::Compound,
			Self::IntArray(_) => Tag::IntArray,
			Self::LongArray(_) => Tag::LongArray,
		}
	}
}

macro_rules! spec_array {
    ($(#[$($attrss:tt)*])* $name:ident ($type:ty)) => {
        $(#[$($attrss)*])*
        #[derive(Debug, Clone, PartialEq)]
		pub struct $name(pub Vec<$type>);

		impl Deref for $name {
			type Target = Vec<$type>;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
		impl DerefMut for $name {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
		impl From<Vec<$type>> for $name {
			fn from(value: Vec<$type>) -> Self {
				Self(value)
			}
		}
		impl From<$name> for Vec<$type> {
			fn from(value: $name) -> Self {
				value.0
			}
		}
    };
}
spec_array! {
	/// NBT ByteArray marker
	NbtByteArray(i8)
}
spec_array! {
	/// NBT IntArray marker
	NbtIntArray(i32)
}
spec_array! {
	/// NBT LongArray marker
	NbtLongArray(i64)
}

macro_rules! gen_from_impls {
	($outer:ident : $variant:ident : $inner:ty) => {
		impl From<$inner> for $outer {
			fn from(value: $inner) -> Self {
				Self::$variant(value)
			}
		}
		impl TryFrom<$outer> for $inner {
			type Error = Tag;

			fn try_from(value: $outer) -> Result<Self, Self::Error> {
				match value {
					$outer::$variant(v) => Ok(v),
					other => Err(other.tag()),
				}
			}
		}
	};
}
gen_from_impls!(NbtValue 	: Byte 		: i8);
gen_from_impls!(NbtValue 	: Short 	: i16);
gen_from_impls!(NbtValue 	: Int 		: i32);
gen_from_impls!(NbtValue 	: Long 		: i64);
gen_from_impls!(NbtValue 	: Float 	: f32);
gen_from_impls!(NbtValue 	: Double 	: f64);
gen_from_impls!(NbtValue 	: ByteArray : NbtByteArray);
gen_from_impls!(NbtValue 	: String 	: NbtString);
gen_from_impls!(NbtValue 	: List 		: NbtList);
gen_from_impls!(NbtValue 	: Compound 	: NbtCompound);
gen_from_impls!(NbtValue 	: IntArray 	: NbtIntArray);
gen_from_impls!(NbtValue 	: LongArray : NbtLongArray);
gen_from_impls!(NbtList 	: Byte 		: Vec<i8>);
gen_from_impls!(NbtList 	: Short 	: Vec<i16>);
gen_from_impls!(NbtList		: Int 		: Vec<i32>);
gen_from_impls!(NbtList 	: Long 		: Vec<i64>);
gen_from_impls!(NbtList 	: Float 	: Vec<f32>);
gen_from_impls!(NbtList 	: Double 	: Vec<f64>);
gen_from_impls!(NbtList 	: ByteArray : Vec<NbtByteArray>);
gen_from_impls!(NbtList 	: String 	: Vec<NbtString>);
gen_from_impls!(NbtList 	: List 		: Vec<NbtList>);
gen_from_impls!(NbtList 	: Compound 	: Vec<NbtCompound>);
gen_from_impls!(NbtList 	: IntArray 	: Vec<NbtIntArray>);
gen_from_impls!(NbtList 	: LongArray : Vec<NbtLongArray>);

macro_rules! gen_from_impls_list_bypass {
	($variant:ident : $inner:ty) => {
		impl From<$inner> for NbtValue {
			fn from(value: $inner) -> Self {
				Self::List(NbtList::$variant(value))
			}
		}
		impl TryFrom<NbtValue> for $inner {
			type Error = Tag;

			fn try_from(value: NbtValue) -> Result<Self, Self::Error> {
				match value {
					NbtValue::List(NbtList::$variant(v)) => Ok(v),
					NbtValue::List(other) => Err(other.tag()),
					other => Err(other.tag()),
				}
			}
		}
	};
}
gen_from_impls_list_bypass!(Byte 		: Vec<i8>);
gen_from_impls_list_bypass!(Short 		: Vec<i16>);
gen_from_impls_list_bypass!(Int 		: Vec<i32>);
gen_from_impls_list_bypass!(Long 		: Vec<i64>);
gen_from_impls_list_bypass!(Float 		: Vec<f32>);
gen_from_impls_list_bypass!(Double 		: Vec<f64>);
gen_from_impls_list_bypass!(ByteArray 	: Vec<NbtByteArray>);
gen_from_impls_list_bypass!(String 		: Vec<NbtString>);
gen_from_impls_list_bypass!(List 		: Vec<NbtList>);
gen_from_impls_list_bypass!(Compound 	: Vec<NbtCompound>);
gen_from_impls_list_bypass!(IntArray 	: Vec<NbtIntArray>);
gen_from_impls_list_bypass!(LongArray 	: Vec<NbtLongArray>);

// Brace for maximum slop
impl NbtValue {
	/// Returns the value of any integer type (byte, short, int, long)
	pub fn as_int_nonstrict(&self) -> Option<i64> {
		match self {
			NbtValue::Byte(v) => Some(*v as i64),
			NbtValue::Short(v) => Some(*v as i64),
			NbtValue::Int(v) => Some(*v as i64),
			NbtValue::Long(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns the value of any integer type (byte, short, int, long)
	///
	/// This is the same as [`NbtValue::as_int_nonstrict()`], but it consumes the NBT.
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
	/// This is the same as [`NbtValue::expect_int_nonstrict()`], but it consumes the NBT.
	pub fn unwrap_int_nonstrict(self) -> i64 {
		self.expect_int_nonstrict()
	}
	/// Returns the value of any float type (float or double)
	pub fn as_float_nonstrict(&self) -> Option<f64> {
		match self {
			NbtValue::Float(v) => Some(*v as f64),
			NbtValue::Double(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns the value of any float type (float or double)
	///
	/// This is the same as [`NbtValue::as_float_nonstrict()`], but it consumes the NBT.
	pub fn into_float_nonstrict(self) -> Option<f64> {
		self.as_float_nonstrict()
	}
	/// Returns the value of any float type (float or double), panicking if it's not a float
	pub fn expect_float_nonstrict(&self) -> f64 {
		match self.as_float_nonstrict() {
			Some(v) => v,
			None => panic!("Expected float, found {:?}", self),
		}
	}
	/// Returns the value of any float type (float or double), panicking if it's not a float
	///
	/// This is the same as [`NbtValue::expect_float_nonstrict()`], but it consumes the NBT.
	pub fn unwrap_float_nonstrict(self) -> f64 {
		self.expect_float_nonstrict()
	}
	/// Returns a Long, if this NBT is a long.
	pub fn as_long(&self) -> Option<i64> {
		match self {
			NbtValue::Long(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns an Int, if this NBT is an int.
	pub fn as_int(&self) -> Option<i32> {
		match self {
			NbtValue::Int(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a Short, if this NBT is a short.
	pub fn as_short(&self) -> Option<i16> {
		match self {
			NbtValue::Short(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a Byte, if this NBT is a byte.
	pub fn as_byte(&self) -> Option<i8> {
		match self {
			NbtValue::Byte(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a Double, if this NBT is a double.
	pub fn as_double(&self) -> Option<f64> {
		match self {
			NbtValue::Double(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a Float, if this NBT is a float.
	pub fn as_float(&self) -> Option<f32> {
		match self {
			NbtValue::Float(v) => Some(*v),
			_ => None,
		}
	}
	/// Returns a String, if this NBT is a string.
	pub fn as_string(&self) -> Option<&NbtString> {
		match self {
			NbtValue::String(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a List, if this NBT is a list.
	pub fn as_list(&self) -> Option<&NbtList> {
		match self {
			NbtValue::List(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a Compound, if this NBT is a compound.
	pub fn as_compound(&self) -> Option<&NbtCompound> {
		match self {
			NbtValue::Compound(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a LongArray, if this NBT is a long array.
	pub fn as_long_array(&self) -> Option<&Vec<i64>> {
		match self {
			NbtValue::LongArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a IntArray, if this NBT is an int array.
	pub fn as_int_array(&self) -> Option<&Vec<i32>> {
		match self {
			NbtValue::IntArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a ByteArray, if this NBT is a byte array.
	pub fn as_byte_array(&self) -> Option<&Vec<i8>> {
		match self {
			NbtValue::ByteArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a long.
	pub fn as_mut_long(&mut self) -> Option<&mut i64> {
		match self {
			NbtValue::Long(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is an int.
	pub fn as_mut_int(&mut self) -> Option<&mut i32> {
		match self {
			NbtValue::Int(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a short.
	pub fn as_mut_short(&mut self) -> Option<&mut i16> {
		match self {
			NbtValue::Short(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a byte.
	pub fn as_mut_byte(&mut self) -> Option<&mut i8> {
		match self {
			NbtValue::Byte(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a double.
	pub fn as_mut_double(&mut self) -> Option<&mut f64> {
		match self {
			NbtValue::Double(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a float.
	pub fn as_mut_float(&mut self) -> Option<&mut f32> {
		match self {
			NbtValue::Float(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a string.
	pub fn as_mut_string(&mut self) -> Option<&mut NbtString> {
		match self {
			NbtValue::String(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a list.
	pub fn as_mut_list(&mut self) -> Option<&mut NbtList> {
		match self {
			NbtValue::List(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a compound.
	pub fn as_mut_compound(&mut self) -> Option<&mut NbtCompound> {
		match self {
			NbtValue::Compound(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a long array.
	pub fn as_mut_long_array(&mut self) -> Option<&mut Vec<i64>> {
		match self {
			NbtValue::LongArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is an int array.
	pub fn as_mut_int_array(&mut self) -> Option<&mut Vec<i32>> {
		match self {
			NbtValue::IntArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a mutable reference to the inner value, if this NBT is a byte array.
	pub fn as_mut_byte_array(&mut self) -> Option<&mut Vec<i8>> {
		match self {
			NbtValue::ByteArray(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a long, if this NBT is a long.
	///
	/// This is the same as [`NbtValue::as_long()`], but it consumes the NBT.
	pub fn into_long(self) -> Option<i64> {
		match self {
			NbtValue::Long(v) => Some(v),
			_ => None,
		}
	}
	/// Returns an int, if this NBT is an int.
	///
	/// This is the same as [`NbtValue::as_int()`], but it consumes the NBT.
	pub fn into_int(self) -> Option<i32> {
		match self {
			NbtValue::Int(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a short, if this NBT is a short.
	///
	/// This is the same as [`NbtValue::as_short()`], but it consumes the NBT.
	pub fn into_short(self) -> Option<i16> {
		match self {
			NbtValue::Short(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a byte, if this NBT is a byte.
	///
	/// This is the same as [`NbtValue::as_byte()`], but it consumes the NBT.
	pub fn into_byte(self) -> Option<i8> {
		match self {
			NbtValue::Byte(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a double, if this NBT is a double.
	///
	/// This is the same as [`NbtValue::as_double()`], but it consumes the NBT.
	pub fn into_double(self) -> Option<f64> {
		match self {
			NbtValue::Double(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a float, if this NBT is a float.
	///
	/// This is the same as [`NbtValue::as_float()`], but it consumes the NBT.
	pub fn into_float(self) -> Option<f32> {
		match self {
			NbtValue::Float(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a string by value, if this NBT is a string.
	pub fn into_string(self) -> Option<NbtString> {
		match self {
			NbtValue::String(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a list by value, if this NBT is a list.
	pub fn into_list(self) -> Option<NbtList> {
		match self {
			NbtValue::List(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a compound by value, if this NBT is a compound.
	pub fn into_compound(self) -> Option<NbtCompound> {
		match self {
			NbtValue::Compound(v) => Some(v),
			_ => None,
		}
	}
	/// Returns a long array by value, if this NBT is a long array.
	pub fn into_long_array(self) -> Option<Vec<i64>> {
		match self {
			NbtValue::LongArray(v) => Some(v.0),
			_ => None,
		}
	}
	/// Returns an int array by value, if this NBT is an int array.
	pub fn into_int_array(self) -> Option<Vec<i32>> {
		match self {
			NbtValue::IntArray(v) => Some(v.0),
			_ => None,
		}
	}
	/// Returns a byte array by value, if this NBT is a byte array.
	pub fn into_byte_array(self) -> Option<Vec<i8>> {
		match self {
			NbtValue::ByteArray(v) => Some(v.0),
			_ => None,
		}
	}
	/// Returns a long, if this NBT is a long, panics otherwise.
	pub fn expect_long(&self) -> i64 {
		match self {
			NbtValue::Long(v) => *v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	/// Returns an int, if this NBT is an int, panics otherwise.
	pub fn expect_int(&self) -> i32 {
		match self {
			NbtValue::Int(v) => *v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	/// Returns a short, if this NBT is a short, panics otherwise.
	pub fn expect_short(&self) -> i16 {
		match self {
			NbtValue::Short(v) => *v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	/// Returns a byte, if this NBT is a byte, panics otherwise.
	pub fn expect_byte(&self) -> i8 {
		match self {
			NbtValue::Byte(v) => *v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	/// Returns a double, if this NBT is a double, panics otherwise.
	pub fn expect_double(&self) -> f64 {
		match self {
			NbtValue::Double(v) => *v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	/// Returns a float, if this NBT is a float, panics otherwise.
	pub fn expect_float(&self) -> f32 {
		match self {
			NbtValue::Float(v) => *v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	/// Returns a string reference, if this NBT is a string, panics otherwise.
	pub fn expect_string(&self) -> &NbtString {
		match self {
			NbtValue::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	/// Returns a byte array reference, if this NBT is a byte array, panics otherwise.
	pub fn expect_list(&self) -> &NbtList {
		match self {
			NbtValue::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	/// Returns a compound reference, if this NBT is a compound, panics otherwise.
	pub fn expect_compound(&self) -> &NbtCompound {
		match self {
			NbtValue::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	/// Returns a long array reference, if this NBT is a long array, panics otherwise.
	pub fn expect_long_array(&self) -> &Vec<i64> {
		match self {
			NbtValue::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	/// Returns an int array reference, if this NBT is an int array, panics otherwise.
	pub fn expect_int_array(&self) -> &Vec<i32> {
		match self {
			NbtValue::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	/// Returns a byte array reference, if this NBT is a byte array, panics otherwise.
	pub fn expect_byte_array(&self) -> &Vec<i8> {
		match self {
			NbtValue::ByteArray(v) => v,
			_ => panic!("Expected ByteArray, found {:?}", self),
		}
	}
	/// Returns a long by mutable reference, if this NBT is a long, panics otherwise.
	pub fn expect_mut_long(&mut self) -> &mut i64 {
		match self {
			NbtValue::Long(v) => v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	/// Returns an int by mutable reference, if this NBT is an int, panics otherwise.
	pub fn expect_mut_int(&mut self) -> &mut i32 {
		match self {
			NbtValue::Int(v) => v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	/// Returns a short by mutable reference, if this NBT is a short, panics otherwise.
	pub fn expect_mut_short(&mut self) -> &mut i16 {
		match self {
			NbtValue::Short(v) => v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	/// Returns a byte by mutable reference, if this NBT is a byte, panics otherwise.
	pub fn expect_mut_byte(&mut self) -> &mut i8 {
		match self {
			NbtValue::Byte(v) => v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	/// Returns a double by mutable reference, if this NBT is a double, panics otherwise.
	pub fn expect_mut_double(&mut self) -> &mut f64 {
		match self {
			NbtValue::Double(v) => v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	/// Returns a float by mutable reference, if this NBT is a float, panics otherwise.
	pub fn expect_mut_float(&mut self) -> &mut f32 {
		match self {
			NbtValue::Float(v) => v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	/// Returns a string by mutable reference, if this NBT is a string, panics otherwise.
	pub fn expect_mut_string(&mut self) -> &mut NbtString {
		match self {
			NbtValue::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	/// Returns a byte array by mutable reference, if this NBT is a list, panics otherwise.
	pub fn expect_mut_list(&mut self) -> &mut NbtList {
		match self {
			NbtValue::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	/// Returns a compound by mutable reference, if this NBT is a compound, panics otherwise.
	pub fn expect_mut_compound(&mut self) -> &mut NbtCompound {
		match self {
			NbtValue::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	/// Returns a long array by mutable reference, if this NBT is a long array, panics otherwise.
	pub fn expect_mut_long_array(&mut self) -> &mut Vec<i64> {
		match self {
			NbtValue::LongArray(v) => v,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	/// Returns an int array by mutable reference, if this NBT is an int array, panics otherwise.
	pub fn expect_mut_int_array(&mut self) -> &mut Vec<i32> {
		match self {
			NbtValue::IntArray(v) => v,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	/// Returns a byte array by mutable reference, if this NBT is a byte array, panics otherwise.
	pub fn expect_mut_byte_array(&mut self) -> &mut Vec<i8> {
		match self {
			NbtValue::ByteArray(v) => v,
			_ => panic!("Expected ByteArray, found {:?}", self),
		}
	}
	/// Returns a long, if this NBT is a long, panics otherwise.
	///
	/// This is the same as [`NbtValue::expect_long()`], but it consumes the NBT.
	pub fn unwrap_long(self) -> i64 {
		match self {
			NbtValue::Long(v) => v,
			_ => panic!("Expected Long, found {:?}", self),
		}
	}
	/// Returns an int, if this NBT is an int, panics otherwise.
	///
	/// This is the same as [`NbtValue::expect_int()`], but it consumes the NBT.
	pub fn unwrap_int(self) -> i32 {
		match self {
			NbtValue::Int(v) => v,
			_ => panic!("Expected Int, found {:?}", self),
		}
	}
	/// Returns a short, if this NBT is a short, panics otherwise.
	///
	/// This is the same as [`NbtValue::expect_short()`], but it consumes the NBT.
	pub fn unwrap_short(self) -> i16 {
		match self {
			NbtValue::Short(v) => v,
			_ => panic!("Expected Short, found {:?}", self),
		}
	}
	/// Returns a byte, if this NBT is a byte, panics otherwise.
	///
	/// This is the same as [`NbtValue::expect_byte()`], but it consumes the NBT.
	pub fn unwrap_byte(self) -> i8 {
		match self {
			NbtValue::Byte(v) => v,
			_ => panic!("Expected Byte, found {:?}", self),
		}
	}
	/// Returns a double, if this NBT is a double, panics otherwise.
	///
	/// This is the same as [`NbtValue::expect_double()`], but it consumes the NBT.
	pub fn unwrap_double(self) -> f64 {
		match self {
			NbtValue::Double(v) => v,
			_ => panic!("Expected Double, found {:?}", self),
		}
	}
	/// Returns a float, if this NBT is a float, panics otherwise.
	///
	/// This is the same as [`NbtValue::expect_float()`], but it consumes the NBT.
	pub fn unwrap_float(self) -> f32 {
		match self {
			NbtValue::Float(v) => v,
			_ => panic!("Expected Float, found {:?}", self),
		}
	}
	/// Returns a string by value, if this NBT is a string, panics otherwise.
	pub fn unwrap_string(self) -> NbtString {
		match self {
			NbtValue::String(v) => v,
			_ => panic!("Expected String, found {:?}", self),
		}
	}
	/// Returns a list by value, if this NBT is a list, panics otherwise.
	pub fn unwrap_list(self) -> NbtList {
		match self {
			NbtValue::List(v) => v,
			_ => panic!("Expected List, found {:?}", self),
		}
	}
	/// Returns a compound by value, if this NBT is a compound, panics otherwise.
	pub fn unwrap_compound(self) -> NbtCompound {
		match self {
			NbtValue::Compound(v) => v,
			_ => panic!("Expected Compound, found {:?}", self),
		}
	}
	/// Returns a long array by value, if this NBT is a long array, panics otherwise.
	pub fn unwrap_long_array(self) -> Vec<i64> {
		match self {
			NbtValue::LongArray(v) => v.0,
			_ => panic!("Expected LongArray, found {:?}", self),
		}
	}
	/// Returns an int array by value, if this NBT is an int array, panics otherwise.
	pub fn unwrap_int_array(self) -> Vec<i32> {
		match self {
			NbtValue::IntArray(v) => v.0,
			_ => panic!("Expected IntArray, found {:?}", self),
		}
	}
	/// Returns a byte array by value, if this NBT is a byte array, panics otherwise.
	pub fn unwrap_byte_array(self) -> Vec<i8> {
		match self {
			NbtValue::ByteArray(v) => v.0,
			_ => panic!("Expected ByteArray, found {:?}", self),
		}
	}
}
