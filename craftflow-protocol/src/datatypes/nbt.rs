use crate::MinecraftProtocol;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::{
	collections::BTreeMap,
	io::{Read, Write},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Nbt<T>(pub T);

impl<'a, T: Deserialize<'a> + Serialize> MinecraftProtocol<'a> for Nbt<T> {
	fn read(protocol_version: u32, input: &'a [u8]) -> crate::Result<(&'a [u8], Self)> {
		todo!()
	}

	fn write(&self, protocol_version: u32, output: &mut impl Write) -> crate::Result<usize> {
		todo!()
	}
}

// /// This is basically the `NBT` compound
// pub struct NbtRoot {
// 	pub compound: BTreeMap<String, NbtValue>,
// }

// pub enum NbtValue {
// 	Byte(i8),
// 	Short(i16),
// 	Int(i32),
// 	Long(i64),
// 	Float(f32),
// 	Double(f64),
// 	ByteArray(Vec<i8>),
// 	String(String),
// 	List(Vec<NbtValue>),
// 	Compound(BTreeMap<String, NbtValue>),
// 	IntArray(Vec<i32>),
// 	LongArray(Vec<i64>),
// }

// impl MinecraftProtocol for NbtRoot {
// 	fn read(protocol_version: u32, input: &mut impl Read) -> Result<Self> {
// 		let tag = input.read_u8()?;
// 		if tag != 10 {
// 			bail!("Expected NBT compound tag, got {}", tag);
// 		}

// 		if protocol_version <= 764 {
// 			// older protocol versions include an empty root tag name
// 			let name_len = input.read_u16::<BigEndian>()?;
// 			let mut name = vec![0; name_len as usize];
// 			input.read_exact(&mut name)?;
// 			// IDK might want to error if name len is not 0 here
// 			// but i guess it wont hurt anyone
// 		}

// 		match NbtValue::read_value::<BigEndian>(input, tag)? {
// 			NbtValue::Compound(compound) => Ok(NbtRoot { compound }),
// 			_ => unreachable!(),
// 		}
// 	}

// 	fn write(&self, protocol_version: u32, output: &mut impl Write) -> Result<usize> {
// 		let mut written = 0;

// 		written += output.write_u8(10).map(|_| 1)?;

// 		if protocol_version <= 764 {
// 			// older protocol versions include an empty root tag name
// 			written += output.write_u16::<BigEndian>(0).map(|_| 2)?;
// 		}

// 		written += write_compound::<BigEndian>(&self.compound, output)?;

// 		Ok(written)
// 	}
// }

// impl MinecraftProtocol for NbtValue {
// 	fn read(_protocol_version: u32, input: &mut impl Read) -> Result<Self> {
// 		let tag = input.read_u8()?;

// 		NbtValue::read_value::<BigEndian>(input, tag)
// 	}

// 	fn write(&self, _protocol_version: u32, output: &mut impl Write) -> Result<usize> {
// 		let mut written = 0;

// 		written += self.write_tag(output)?;
// 		written += self.write_value::<BigEndian>(output)?;

// 		Ok(written)
// 	}
// }

// impl NbtValue {
// 	fn write_tag(&self, output: &mut impl Write) -> Result<usize> {
// 		Ok(match self {
// 			NbtValue::Byte(_) => output.write_u8(1),
// 			NbtValue::Short(_) => output.write_u8(2),
// 			NbtValue::Int(_) => output.write_u8(3),
// 			NbtValue::Long(_) => output.write_u8(4),
// 			NbtValue::Float(_) => output.write_u8(5),
// 			NbtValue::Double(_) => output.write_u8(6),
// 			NbtValue::ByteArray(_) => output.write_u8(7),
// 			NbtValue::String(_) => output.write_u8(8),
// 			NbtValue::List(_) => output.write_u8(9),
// 			NbtValue::Compound(_) => output.write_u8(10),
// 			NbtValue::IntArray(_) => output.write_u8(11),
// 			NbtValue::LongArray(_) => output.write_u8(12),
// 		}
// 		.map(|_| 1)?)
// 	}
// 	fn write_value<B: ByteOrder>(&self, output: &mut impl Write) -> Result<usize> {
// 		match self {
// 			NbtValue::Byte(v) => {
// 				output.write_i8(*v)?;
// 				Ok(1)
// 			}
// 			NbtValue::Short(v) => {
// 				output.write_i16::<B>(*v)?;
// 				Ok(2)
// 			}
// 			NbtValue::Int(v) => {
// 				output.write_i32::<B>(*v)?;
// 				Ok(4)
// 			}
// 			NbtValue::Long(v) => {
// 				output.write_i64::<B>(*v)?;
// 				Ok(8)
// 			}
// 			NbtValue::Float(v) => {
// 				output.write_f32::<B>(*v)?;
// 				Ok(4)
// 			}
// 			NbtValue::Double(v) => {
// 				output.write_f64::<B>(*v)?;
// 				Ok(8)
// 			}
// 			NbtValue::ByteArray(v) => {
// 				output.write_i32::<B>(v.len() as i32)?;
// 				for &b in v {
// 					output.write_i8(b)?;
// 				}
// 				Ok(4 + v.len())
// 			}
// 			NbtValue::String(v) => {
// 				output.write_u16::<B>(v.len() as u16)?;
// 				output.write_all(v.as_bytes())?;
// 				Ok(2 + v.len())
// 			}
// 			NbtValue::List(v) => {
// 				// write the tag of the type of values in the list
// 				match v.get(0) {
// 					Some(e) => {
// 						e.write_tag(output)?;
// 					}
// 					None => output.write_u8(0)?, // empty list, can use any tag
// 				}

// 				output.write_i32::<B>(v.len() as i32)?;
// 				let mut len = 5;
// 				for value in v {
// 					len += value.write(0, output)?;
// 				}
// 				Ok(len)
// 			}
// 			NbtValue::Compound(v) => write_compound::<B>(v, output),
// 			NbtValue::IntArray(v) => {
// 				output.write_i32::<B>(v.len() as i32)?;
// 				for &i in v {
// 					output.write_i32::<B>(i)?;
// 				}
// 				Ok(4 + 4 * v.len())
// 			}
// 			NbtValue::LongArray(v) => {
// 				output.write_i32::<B>(v.len() as i32)?;
// 				for &i in v {
// 					output.write_i64::<B>(i)?;
// 				}
// 				Ok(4 + 8 * v.len())
// 			}
// 		}
// 	}
// 	fn read_value<B: ByteOrder>(input: &mut impl Read, tag: u8) -> Result<Self> {
// 		match tag {
// 			1 => Ok(NbtValue::Byte(input.read_i8()?)),
// 			2 => Ok(NbtValue::Short(input.read_i16::<B>()?)),
// 			3 => Ok(NbtValue::Int(input.read_i32::<B>()?)),
// 			4 => Ok(NbtValue::Long(input.read_i64::<B>()?)),
// 			5 => Ok(NbtValue::Float(input.read_f32::<B>()?)),
// 			6 => Ok(NbtValue::Double(input.read_f64::<B>()?)),
// 			7 => {
// 				let len = input.read_i32::<B>()?;
// 				let mut buf = vec![0u8; len as usize];
// 				input.read_exact(&mut buf[..])?;
// 				// i trust compiler will optimize 🙏😤
// 				let buf = buf.into_iter().map(|v| v as i8).collect();
// 				Ok(NbtValue::ByteArray(buf))
// 			}
// 			8 => {
// 				let len = input.read_u16::<B>()?;
// 				let mut buf = vec![0; len as usize];
// 				input.read_exact(&mut buf)?;
// 				Ok(NbtValue::String(String::from_utf8(buf)?))
// 			}
// 			9 => {
// 				let list_tag = input.read_u8()?;
// 				let len = input.read_i32::<B>()?;
// 				let mut buf = Vec::with_capacity(len as usize);
// 				for _ in 0..len {
// 					buf.push(NbtValue::read_value::<B>(input, list_tag)?);
// 				}
// 				Ok(NbtValue::List(buf))
// 			}
// 			10 => {
// 				let mut buf = BTreeMap::new();
// 				loop {
// 					let tag = input.read_u8()?;
// 					if tag == 0 {
// 						break;
// 					}
// 					let name_len = input.read_u16::<B>()?;
// 					let mut name_buf = vec![0; name_len as usize];
// 					input.read_exact(&mut name_buf)?;
// 					let name = String::from_utf8(name_buf)?;
// 					let value = NbtValue::read_value::<B>(input, tag)?;
// 					buf.insert(name, value);
// 				}
// 				Ok(NbtValue::Compound(buf))
// 			}
// 			11 => {
// 				let len = input.read_i32::<B>()?;
// 				let mut buf = Vec::with_capacity(len as usize);
// 				for _ in 0..len {
// 					buf.push(input.read_i32::<B>()?);
// 				}
// 				Ok(NbtValue::IntArray(buf))
// 			}
// 			12 => {
// 				let len = input.read_i32::<B>()?;
// 				let mut buf = Vec::with_capacity(len as usize);
// 				for _ in 0..len {
// 					buf.push(input.read_i64::<B>()?);
// 				}
// 				Ok(NbtValue::LongArray(buf))
// 			}
// 			_ => Err(anyhow::anyhow!("Invalid NBT tag: {}", tag)),
// 		}
// 	}
// }

// // Writes a compound without the tag
// fn write_compound<B: ByteOrder>(
// 	compound: &BTreeMap<String, NbtValue>,
// 	output: &mut impl Write,
// ) -> Result<usize> {
// 	let mut len = 0;
// 	for (key, value) in compound {
// 		output.write_u16::<B>(key.len() as u16)?;
// 		len += 2;
// 		output.write_all(key.as_bytes())?;
// 		len += key.len();

// 		len += value.write_tag(output)?;
// 		len += value.write_value::<B>(output)?;
// 	}
// 	output.write_u8(0)?;
// 	Ok(len + 1)
// }

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_read_nbt() {}
// }
