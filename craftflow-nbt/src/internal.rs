use std::collections::HashMap;

use crate::Tag;
use crate::{
	Result,
	nbtvalue::{NbtByteArray, NbtCompound, NbtIntArray, NbtList, NbtLongArray},
};
use generic_array::ArrayLength;
use typenum::{U1, U2, U4, U8};

pub use typenum::U0;

pub mod read;
pub mod write;

pub trait InternalNbt {
	const TAG: Tag;

	/// if true that means always fixed size and any bit combo is valid!!
	const IS_POD: bool;
	type StaticSize: ArrayLength;
}
pub trait InternalNbtRead: InternalNbt + Sized {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self>;
}
pub trait InternalNbtWrite: InternalNbt {
	fn nbt_iwrite(&self, output: &mut Vec<u8>) -> usize;
}

macro_rules! static_nbt {
    ($($tag:ident = $type:ty = $size:ty),*) => {$(
        impl InternalNbt for $type {
        	const TAG: Tag = Tag::$tag;

        	const IS_POD: bool = true;
        	type StaticSize = $size;
        }
    )*};
}
static_nbt!(
	Byte = i8 = U1,
	Short = i16 = U2,
	Int = i32 = U4,
	Long = i64 = U8,
	Float = f32 = U4,
	Double = f64 = U8
);

macro_rules! dynamic_nbt {
    ($($tag:ident = $type:ty),*) => {$(
        impl InternalNbt for $type {
        	const TAG: Tag = Tag::$tag;

        	const IS_POD: bool = false;
        	type StaticSize = U0;
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
	type StaticSize = U0;
}
impl<T: InternalNbt> InternalNbt for HashMap<String, T> {
	const TAG: Tag = Tag::Compound;

	const IS_POD: bool = false;
	type StaticSize = U0;
}
