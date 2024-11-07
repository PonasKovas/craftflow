mod tests;

pub use shallowclone_derive::ShallowClone;

/// The same as [`Clone`], but doesnt clone `Cow`s, instead it just borrows them.
pub trait ShallowClone<'a> {
	type Target;

	fn shallow_clone(&'a self) -> Self::Target;
}

use std::{borrow::Cow, collections::HashMap, hash::Hash, marker::PhantomData};

impl<'a, 'b, T: ToOwned + ?Sized> ShallowClone<'a> for Cow<'b, T>
where
	'b: 'a,
{
	type Target = Cow<'a, T>;

	fn shallow_clone(&'a self) -> Self::Target {
		Cow::Borrowed(&**self)
	}
}

impl<'a, 'b, T: ?Sized> ShallowClone<'a> for &'b T
where
	'b: 'a,
{
	type Target = &'a T;

	fn shallow_clone(&'a self) -> Self::Target {
		self
	}
}

impl<'a, T> ShallowClone<'a> for PhantomData<T> {
	type Target = Self;

	fn shallow_clone(&'a self) -> Self::Target {
		*self
	}
}

macro_rules! impl_by_clone {
    ($( $x:ty ),* $(,)? ) => {
        $(
            impl<'a> ShallowClone<'a> for $x {
                type Target = Self;

                fn shallow_clone(&'a self) -> Self::Target {
                    self.clone()
                }
            }
        )*
    };
}

// primitives
impl_by_clone! { u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, bool, char}

// common std types
impl_by_clone! { String }

impl<'a, T: ShallowClone<'a>> ShallowClone<'a> for Option<T> {
	type Target = Option<T::Target>;

	fn shallow_clone(&'a self) -> Self::Target {
		self.as_ref().map(|x| x.shallow_clone())
	}
}
impl<'a, T: ShallowClone<'a>> ShallowClone<'a> for Vec<T> {
	type Target = Vec<T::Target>;

	fn shallow_clone(&'a self) -> Self::Target {
		self.iter().map(|x| x.shallow_clone()).collect()
	}
}
impl<'a, T: ShallowClone<'a>> ShallowClone<'a> for Box<T> {
	type Target = Box<T::Target>;

	fn shallow_clone(&'a self) -> Self::Target {
		Box::new(self.as_ref().shallow_clone())
	}
}
impl<'a, K: ShallowClone<'a>, V: ShallowClone<'a>> ShallowClone<'a> for HashMap<K, V>
where
	K::Target: Eq + Hash,
{
	type Target = HashMap<K::Target, V::Target>;

	fn shallow_clone(&'a self) -> Self::Target {
		self.iter()
			.map(|(k, v)| (k.shallow_clone(), v.shallow_clone()))
			.collect()
	}
}
