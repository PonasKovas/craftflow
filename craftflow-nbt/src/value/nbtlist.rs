use super::{FallibleBuf, NbtArray, NbtString, Tag};
use crate::Result;
use bytes::Bytes;
use std::{
	io::{self, Write},
	marker::PhantomData,
};

pub enum NbtList {
	Empty,
	Byte(NbtArray<i8>),
	Short(NbtArray<i16>),
	Int(NbtArray<i32>),
	Long(NbtArray<i64>),
	Float(NbtArray<f32>),
	Double(NbtArray<f64>),
	String(NbtArray<NbtString>),
	ByteArray(NbtArray<NbtArray<i8>>),
	IntArray(NbtArray<NbtArray<i32>>),
	LongArray(NbtArray<NbtArray<i64>>),
	List(NbtArray<NbtList>),
}

impl NbtList {
	pub fn to_iter(&self) -> () {
		todo!()
	}
}

impl NbtList {
	pub(crate) fn parse(data: &mut Bytes) -> Result<Self> {
		let tag = Tag::new(data.fget_u8()?)?;

		let len = data.fget_u32()? as usize;

		// TAG_END only allowed as type in 0-length lists
		if tag == Tag::End && len != 0 {
			return Err(crate::Error::UnexpectedTagEnd);
		}

		match tag {
			Tag::End => Self::Empty,
			Tag::Byte => Self::Byte(NbtArray::<i8>::parse(data)?),
			Tag::Short => Self::Short(NbtArray::<i16>::parse(data)?),
			Tag::Int => Self::Int(NbtArray::<i32>::parse(data)?),
			Tag::Long => Self::Long(NbtArray::<i64>::parse(data)?),
			Tag::Float => Self::Float(NbtArray::<f32>::parse(data)?),
			Tag::Double => Self::Double(NbtArray::<f64>::parse(data)?),
			Tag::ByteArray => Self::ByteArray(NbtArray::<NbtArray<i8>>::parse(data)?),
			Tag::String => Self::String(NbtArray::<NbtString>::parse(data)?),
			Tag::List => Self::List(NbtArray::<NbtList>::parse(data)?),
			Tag::Compound => todo!(),
			Tag::IntArray => Self::IntArray(NbtArray::<NbtArray<i32>>::parse(data)?),
			Tag::LongArray => Self::LongArray(NbtArray::<NbtArray<i64>>::parse(data)?),
		};

		let list = Self {
			data: data.clone(),
			tag,
			len,
		};

		// Traverse the list to validate all internal structures
		list.fallible_iter().validate()?;

		Ok(list)
	}
	pub(crate) fn write(&self, mut output: impl Write) -> io::Result<usize> {
		output.write_all(&[self.tag as u8])?;

		let len = self.len;
		output.write_all(&(len as u32).to_be_bytes())?;

		output.write_all(&*self.data)?;

		Ok(len + 5)
	}
	fn fallible_iter(&self) -> InternalIterEnum {
		match self.tag {
			Tag::End => InternalIterEnum::Empty,
			Tag::Byte => InternalIterEnum::Byte(InternalIter::new(&self.data, self.len)),
			Tag::Short => InternalIterEnum::Short(InternalIter::new(&self.data, self.len)),
			Tag::Int => InternalIterEnum::Int(InternalIter::new(&self.data, self.len)),
			Tag::Long => InternalIterEnum::Long(InternalIter::new(&self.data, self.len)),
			Tag::Float => InternalIterEnum::Float(InternalIter::new(&self.data, self.len)),
			Tag::Double => InternalIterEnum::Double(InternalIter::new(&self.data, self.len)),
			Tag::ByteArray => InternalIterEnum::ByteArray(InternalIter::new(&self.data, self.len)),
			Tag::String => InternalIterEnum::String(InternalIter::new(&self.data, self.len)),
			Tag::List => InternalIterEnum::List(InternalIter::new(&self.data, self.len)),
			Tag::Compound => todo!(),
			Tag::IntArray => InternalIterEnum::IntArray(InternalIter::new(&self.data, self.len)),
			Tag::LongArray => InternalIterEnum::LongArray(InternalIter::new(&self.data, self.len)),
		}
	}
}

enum InternalIterEnum {
	Empty,
	Byte(InternalIter<i8>),
	Short(InternalIter<i16>),
	Int(InternalIter<i32>),
	Long(InternalIter<i64>),
	Float(InternalIter<f32>),
	Double(InternalIter<f64>),
	String(InternalIter<NbtString>),
	ByteArray(InternalIter<NbtArray<i8>>),
	IntArray(InternalIter<NbtArray<i32>>),
	LongArray(InternalIter<NbtArray<i64>>),
	List(InternalIter<NbtList>),
}
impl InternalIterEnum {
	fn validate(self) -> Result<()> {
		match self {
			InternalIterEnum::Empty => Ok(()),
			// basically iterates over the whole iterator and folds all results,
			InternalIterEnum::Byte(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::Short(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::Int(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::Long(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::Float(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::Double(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::String(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::ByteArray(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::IntArray(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::LongArray(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
			InternalIterEnum::List(mut internal_iter) => {
				internal_iter.try_fold((), |_, r| r.map(|_| ()))
			}
		}
	}
}

pub struct InternalIter<T> {
	data: Bytes,
	to_go: usize,
	_phantom: PhantomData<fn(T) -> T>,
}
impl<T> InternalIter<T> {
	fn new(data: &Bytes, len: usize) -> Self {
		Self {
			data: data.clone(),
			to_go: len,
			_phantom: PhantomData,
		}
	}
}

macro_rules! internal_iterator_basic {
	($which:ty, $method:ident) => {
		impl Iterator for InternalIter<$which> {
			type Item = Result<$which>;

			fn next(&mut self) -> Option<Self::Item> {
				if self.to_go == 0 {
					return None;
				}
				self.to_go -= 1;

				Some(self.data.$method().map(|e| e as $which))
			}
		}
	};
}
internal_iterator_basic!(i8, fget_u8);
internal_iterator_basic!(i16, fget_u16);
internal_iterator_basic!(i32, fget_u32);
internal_iterator_basic!(i64, fget_u64);
internal_iterator_basic!(f32, fget_u32);
internal_iterator_basic!(f64, fget_u64);

impl Iterator for InternalIter<NbtString> {
	type Item = Result<NbtString>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.to_go == 0 {
			return None;
		}
		self.to_go -= 1;

		Some(NbtString::parse(&mut self.data))
	}
}

macro_rules! internal_iterator_arr {
	($which:ty) => {
		impl Iterator for InternalIter<NbtArray<$which>> {
			type Item = Result<NbtArray<$which>>;

			fn next(&mut self) -> Option<Self::Item> {
				if self.to_go == 0 {
					return None;
				}
				self.to_go -= 1;

				Some(NbtArray::<$which>::parse(&mut self.data))
			}
		}
	};
}
internal_iterator_arr!(i8);
internal_iterator_arr!(i32);
internal_iterator_arr!(i64);

impl Iterator for InternalIter<NbtList> {
	type Item = Result<NbtList>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.to_go == 0 {
			return None;
		}
		self.to_go -= 1;

		Some(NbtList::parse(&mut self.data))
	}
}
