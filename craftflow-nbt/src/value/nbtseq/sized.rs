use super::NbtSeq;
use crate::casts::*;
use crate::{advance, nbt_format::NbtFormat, Error, Result};
use bytemuck::{cast_slice, cast_slice_mut};
use bytes::{Buf, Bytes};
use std::mem::size_of;
use std::ops::Deref;

macro_rules! gen_impls {
	($which:ty) => {
		impl NbtSeq<$which> {
			pub fn from_slice(data: impl AsRef<[$which]>) -> Self {
				Self::from_vec(data.as_ref().to_owned())
			}
			pub fn from_vec(data: Vec<$which>) -> Self {
				let bytes = data.into_vec_byte_arr();

				Self::new_raw(Bytes::from(bytes))
			}
			pub fn as_slice(&self) -> &[$which] {
				let slice: &[$which] = cast_slice(&*self.data);

				slice
			}
		}
		impl Deref for NbtSeq<$which> {
			type Target = [$which];

			fn deref(&self) -> &Self::Target {
				self.as_slice()
			}
		}
	};
}
gen_impls!(i8);
gen_impls!(i16);
gen_impls!(i32);
gen_impls!(i64);
gen_impls!(f32);
gen_impls!(f64);
