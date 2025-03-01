use crate::tag::Tag;
use crate::{Error, Result, nbt_format::NbtFormat};
use bytes::BytesMut;

// pub use nbtarray::{NbtByteArray, NbtIntArray, NbtLongArray};
// pub use nbtlist::NbtList;
// pub use nbtseq::NbtSeq;
pub use nbtstring::NbtString;

// mod nbtarray;
// mod nbtlist;
mod nbtseq;
mod nbtstring;

pub enum NbtValue {
	Byte(i8),
	Short(i16),
	Int(i32),
	Long(i64),
	Float(f32),
	Double(f64),
	// ByteArray(NbtByteArray),
	String(NbtString),
	// List(NbtList),
	// Compound,
	// IntArray(NbtIntArray),
	// LongArray(NbtLongArray),
}

// impl<O: OptimizedFor> NbtValue<O> {
// 	pub fn read(data: &mut BytesMut) -> Result<Self> {
// 		let tag = Tag::validparse(data)?;

// 		Self::read_inner(data, tag).and_then(|opt| opt.ok_or(Error::UnexpectedNone))
// 	}
// 	pub fn read_optional(data: &mut BytesMut) -> Result<Option<Self>> {
// 		let tag = Tag::validparse(data)?;

// 		Self::read_inner(data, tag)
// 	}
// 	pub fn read_named<O2: OptimizedFor>(data: &mut BytesMut) -> Result<(NbtString<O2>, Self)> {
// 		let tag = Tag::validparse(data)?;

// 		// Read name
// 		let name = NbtString::validparse(data)?;

// 		Self::read_inner(data, tag)
// 			.and_then(|opt| opt.ok_or(Error::UnexpectedNone))
// 			.map(|val| (name, val))
// 	}
// 	pub fn read_optional_named<O2: OptimizedFor>(
// 		data: &mut BytesMut,
// 	) -> Result<(NbtString<O2>, Option<Self>)> {
// 		let tag = Tag::validparse(data)?;

// 		// Read name
// 		let name = NbtString::validparse(data)?;

// 		Self::read_inner(data, tag).map(|val| (name, val))
// 	}
// 	fn read_inner(data: &mut BytesMut, tag: Tag) -> Result<Option<Self>> {
// 		Ok(Some(match tag {
// 			Tag::End => return Ok(None),
// 			Tag::Byte => Self::Byte(i8::validparse(data)?),
// 			Tag::Short => Self::Short(i16::validparse(data)?),
// 			Tag::Int => Self::Int(i32::validparse(data)?),
// 			Tag::Long => Self::Long(i64::validparse(data)?),
// 			Tag::Float => Self::Float(f32::validparse(data)?),
// 			Tag::Double => Self::Double(f64::validparse(data)?),
// 			Tag::ByteArray => todo!(), //Self::ByteArray(NbtByteArray::parse(data)?),
// 			Tag::String => Self::String(NbtString::validparse(data)?),
// 			Tag::List => todo!(), //Self::List(NbtList::parse(data)?),
// 			Tag::Compound => todo!(),
// 			Tag::IntArray => todo!(),  //Self::IntArray(NbtIntArray::parse(data)?),
// 			Tag::LongArray => todo!(), //Self::LongArray(NbtLongArray::parse(data)?),
// 		}))
// 	}

// 	pub fn write(&self, output: &mut Vec<u8>) -> usize {
// 		let mut written = 0;
// 		written += self.tag().write(output);

// 		written += self.write_inner(output);

// 		written
// 	}
// 	pub fn write_named<O2: OptimizedFor>(
// 		&self,
// 		name: &NbtString<O2>,
// 		output: &mut Vec<u8>,
// 	) -> usize {
// 		let mut written = 0;
// 		written += self.tag().write(output);

// 		// Write name
// 		written += name.write(output);

// 		written += self.write_inner(output);

// 		written
// 	}
// 	fn write_inner(&self, output: &mut Vec<u8>) -> usize {
// 		match self {
// 			NbtValue::Byte(d) => d.write(output),
// 			NbtValue::Short(d) => d.write(output),
// 			NbtValue::Int(d) => d.write(output),
// 			NbtValue::Long(d) => d.write(output),
// 			NbtValue::Float(d) => d.write(output),
// 			NbtValue::Double(d) => d.write(output),
// 			NbtValue::String(nbt_string) => nbt_string.write(output),
// 			// NbtValue::ByteArray(nbt_array) => nbt_array.write(output)?,
// 			// NbtValue::IntArray(nbt_array) => nbt_array.write(output)?,
// 			// NbtValue::LongArray(nbt_array) => nbt_array.write(output)?,
// 			// NbtValue::List(nbt_list) => nbt_list.write(output)?,
// 		}
// 	}
// }

// impl<O> NbtValue<O> {
// 	fn tag(&self) -> Tag {
// 		match self {
// 			NbtValue::Byte(_) => Tag::Byte,
// 			NbtValue::Short(_) => Tag::Short,
// 			NbtValue::Int(_) => Tag::Int,
// 			NbtValue::Long(_) => Tag::Long,
// 			NbtValue::Float(_) => Tag::Float,
// 			NbtValue::Double(_) => Tag::Double,
// 			NbtValue::String(_) => Tag::String,
// 			// NbtValue::ByteArray(_) => Tag::ByteArray,
// 			// NbtValue::List(_) => Tag::List,
// 			// NbtValue::IntArray(_) => Tag::IntArray,
// 			// NbtValue::LongArray(_) => Tag::LongArray,
// 		}
// 	}
// }
