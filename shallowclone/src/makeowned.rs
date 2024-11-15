use std::{
	borrow::Cow,
	collections::{BTreeMap, HashMap},
	hash::Hash,
	marker::PhantomData,
};

mod tests;

/// Takes a value and transforms it to be `'static`, cloning parts if necessary
pub trait MakeOwned: Clone {
	/// This must be a `'static` SUBTYPE of `Self`.
	///
	/// For more information see <https://doc.rust-lang.org/reference/subtyping.html>
	type Owned: Clone + 'static;

	fn make_owned(self) -> Self::Owned;
}

impl<'a> MakeOwned for Cow<'a, str> {
	type Owned = Cow<'static, str>;

	fn make_owned(self) -> <Self as MakeOwned>::Owned {
		Cow::Owned(match self {
			Cow::Borrowed(bor) => bor.to_string(),
			Cow::Owned(owned) => owned,
		})
	}
}
impl<'a, T: MakeOwned + Clone> MakeOwned for Cow<'a, [T]>
where
	<T as MakeOwned>::Owned: Clone,
{
	type Owned = Cow<'static, [<T as MakeOwned>::Owned]>;

	fn make_owned(self) -> <Self as MakeOwned>::Owned {
		Cow::Owned(match self {
			Cow::Borrowed(bor) => bor.into_iter().map(|e| e.clone().make_owned()).collect(),
			Cow::Owned(owned) => owned.into_iter().map(|e| e.make_owned()).collect(),
		})
	}
}
impl<'a, A: MakeOwned + 'static, T: Clone + ?Sized> MakeOwned for Cow<'a, T>
where
	T: MakeOwned<Owned = A>,
{
	type Owned = Cow<'static, A>;

	fn make_owned(self) -> <Self as MakeOwned>::Owned {
		match self {
			Cow::Borrowed(bor) => Cow::Owned(bor.clone().make_owned()),
			Cow::Owned(owned) => Cow::Owned(owned.make_owned()),
		}
	}
}

impl<T: 'static> MakeOwned for PhantomData<T> {
	type Owned = Self;

	fn make_owned(self) -> Self::Owned {
		self
	}
}

macro_rules! impl_makeowned_basic {
    ($( $x:ty ),* $(,)? ) => {
        $(
            impl MakeOwned for $x {
                type Owned = Self;

                fn make_owned(self) -> Self::Owned {
                    self
                }
            }
        )*
    };
}

// primitives
impl_makeowned_basic! { u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, bool, char}

impl<const N: usize, T: MakeOwned> MakeOwned for [T; N] {
	type Owned = [T::Owned; N];

	fn make_owned(self) -> Self::Owned {
		self.map(|i| i.make_owned())
	}
}

// common std types
impl_makeowned_basic! { String }

impl<T: MakeOwned> MakeOwned for Option<T> {
	type Owned = Option<T::Owned>;

	fn make_owned(self) -> Self::Owned {
		self.map(|x| x.make_owned())
	}
}

impl<T: MakeOwned> MakeOwned for Vec<T> {
	type Owned = Vec<T::Owned>;

	fn make_owned(self) -> Self::Owned {
		self.into_iter().map(|x| x.make_owned()).collect()
	}
}

impl<T: MakeOwned> MakeOwned for Box<T> {
	type Owned = Box<T::Owned>;

	fn make_owned(self) -> Self::Owned {
		Box::new((*self).make_owned())
	}
}

impl<K: MakeOwned, V: MakeOwned> MakeOwned for HashMap<K, V>
where
	K::Owned: Eq + Hash,
{
	type Owned = HashMap<K::Owned, V::Owned>;

	fn make_owned(self) -> Self::Owned {
		self.into_iter()
			.map(|(k, v)| (k.make_owned(), v.make_owned()))
			.collect()
	}
}

impl<K: MakeOwned, V: MakeOwned> MakeOwned for BTreeMap<K, V>
where
	K::Owned: Eq + Ord,
{
	type Owned = BTreeMap<K::Owned, V::Owned>;

	fn make_owned(self) -> Self::Owned {
		self.into_iter()
			.map(|(k, v)| (k.make_owned(), v.make_owned()))
			.collect()
	}
}

#[cfg(feature = "indexmap")]
impl<K: MakeOwned, V: MakeOwned> MakeOwned for indexmap::IndexMap<K, V>
where
	K::Owned: Hash + Eq,
{
	type Owned = indexmap::IndexMap<K::Owned, V::Owned>;

	fn make_owned(self) -> Self::Owned {
		self.into_iter()
			.map(|(k, v)| (k.make_owned(), v.make_owned()))
			.collect()
	}
}
