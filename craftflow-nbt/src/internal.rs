use crate::Tag;
use crate::{
	Result,
	nbtvalue::{NbtByteArray, NbtCompound, NbtIntArray, NbtList, NbtLongArray},
};
use std::collections::HashMap;

pub mod read;
mod swap_endian;
pub mod write;

pub trait InternalNbt {
	const TAG: Tag;

	/// if true that means always fixed size and any bit combo is valid!!
	const IS_POD: bool;
}
pub trait InternalNbtRead: InternalNbt + Sized {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self>;
}
pub trait InternalNbtWrite: InternalNbt {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize;
}

macro_rules! static_nbt {
    ($($tag:ident = $type:ty),*) => {$(
        impl InternalNbt for $type {
        	const TAG: Tag = Tag::$tag;

        	const IS_POD: bool = true;
        }
    )*};
}
static_nbt!(
	Byte = i8,
	Short = i16,
	Int = i32,
	Long = i64,
	Float = f32,
	Double = f64
);

macro_rules! dynamic_nbt {
    ($($tag:ident = $type:ty),*) => {$(
        impl InternalNbt for $type {
        	const TAG: Tag = Tag::$tag;

        	const IS_POD: bool = false;
        }
    )*};
}
dynamic_nbt!(
	String = String,
	List = NbtList,
	ByteArray = NbtByteArray,
	IntArray = NbtIntArray,
	LongArray = NbtLongArray,
	Compound = NbtCompound
);

impl<T: InternalNbt> InternalNbt for Vec<T> {
	const TAG: Tag = Tag::List;

	const IS_POD: bool = false;
}
impl<T: InternalNbt> InternalNbt for HashMap<String, T> {
	const TAG: Tag = Tag::Compound;

	const IS_POD: bool = false;
}
