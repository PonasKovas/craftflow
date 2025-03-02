use crate::{
	nbtvalue::{NbtByteArray, NbtCompound, NbtIntArray, NbtList, NbtLongArray},
	tag::Tag,
	Result,
};
use generic_array::ArrayLength;
use typenum::{U0, U1, U2, U4, U8};

pub(crate) mod read;
pub(crate) mod write;

pub(crate) trait InternalNbt {
	const TAG: Tag;

	/// if true that means always fixed size and any bit combo is valid!!
	const IS_POD: bool;
	type StaticSize: ArrayLength;
}
pub(crate) trait InternalNbtRead: InternalNbt + Sized {
	fn nbt_iread(input: &mut &[u8]) -> Result<Self>;
}
pub(crate) trait InternalNbtWrite: InternalNbt {
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
