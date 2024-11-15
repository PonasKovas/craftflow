use std::{
	array,
	borrow::Cow,
	collections::{BTreeMap, HashMap},
	hash::Hash,
	marker::PhantomData,
};

mod tests;

/// The same as [`Clone`], but doesnt clone [`Cow`][std::borrow::Cow] values, instead it just borrows them.
pub trait ShallowClone<'a> {
	type Target;

	fn shallow_clone(&'a self) -> Self::Target;
}

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

macro_rules! impl_shallowclone_by_clone {
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
impl_shallowclone_by_clone! { u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, bool, char}

impl<'a, const N: usize, T: ShallowClone<'a>> ShallowClone<'a> for [T; N] {
	type Target = [T::Target; N];

	fn shallow_clone(&'a self) -> Self::Target {
		array::from_fn(|i| self[i].shallow_clone())
	}
}

// common std types
impl_shallowclone_by_clone! { String }

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

impl<'a, K: ShallowClone<'a>, V: ShallowClone<'a>> ShallowClone<'a> for BTreeMap<K, V>
where
	K::Target: Eq + Ord,
{
	type Target = BTreeMap<K::Target, V::Target>;

	fn shallow_clone(&'a self) -> Self::Target {
		self.iter()
			.map(|(k, v)| (k.shallow_clone(), v.shallow_clone()))
			.collect()
	}
}

#[cfg(feature = "indexmap")]
impl<'a, K: ShallowClone<'a>, V: ShallowClone<'a>> ShallowClone<'a> for indexmap::IndexMap<K, V>
where
	K::Target: Hash + Eq,
{
	type Target = indexmap::IndexMap<K::Target, V::Target>;

	fn shallow_clone(&'a self) -> Self::Target {
		self.iter()
			.map(|(k, v)| (k.shallow_clone(), v.shallow_clone()))
			.collect()
	}
}
