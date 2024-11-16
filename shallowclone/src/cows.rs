//! [`Cow`][std::borrow::Cow] alternatives that work well with [`ShallowClone`][crate::ShallowClone].

// standard Cow doesn't work well with shallow clone, since it's invariant over T,
// which introduces problems when T has a lifetime param. If you use Cow you would be forced
// to have two lifetime parameters basically everywhere for no reason, because
// when you shallow clone it you shorten the lifetime of the cow, but can't shorten the lifetime
// of the inner T, and so you end up with two different lifetimes. The following cow implementations
// are simpler, not relying on the ToOwned trait and are covariant over T, therefore not having
// this problem.

use std::{
	borrow::{Borrow, Cow},
	fmt::{Display, Formatter},
	ops::Deref,
};

use crate::{MakeOwned, ShallowClone};

/// Covariant copy-on-write. This is a simpler version of [`Cow`][std::borrow::Cow] that doesn't
/// rely on [`ToOwned`] trait and is covariant over `T`.
///
/// You may wish to use this instead of the standard [`Cow`][std::borrow::Cow] if your
/// inner type `T` contains references. Standard [`Cow<T>`][std::borrow::Cow] is invariant over `T`,
/// which means you can't subtype the lifetimes of the inner `T` when making a shallow clone, which
/// may introduce problems and force you to use multiple lifetimes.
///
/// This is a general version, if you wish to replicate [`Cow<'a, [T]>`][std::borrow::Cow] you
/// should consider using [`CoCowSlice`], which allows you to have slices without an underlying
/// allocated type like [`Vec`][std::vec::Vec].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CoCow<'a, T> {
	Owned(T),
	#[cfg_attr(feature = "serde", serde(skip_deserializing))]
	Borrowed(&'a T),
}

/// Covariant copy-on-write slice. This is a specialised version of [`CoCow`] for slices
/// and allows you to have slices without an underlying allocated type like Vec if you wish.
///
/// You may wish to use this instead of the standard [`Cow`][std::borrow::Cow] if your
/// inner type `T` contains references. Standard [`Cow<T>`][std::borrow::Cow] is invariant over `T`, which means
/// you can't subtype the lifetimes of the inner `T` when making a shallow clone, which
/// may introduce problems and force you to use multiple lifetimes.
///
/// For a more general version, see [`CoCow`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CoCowSlice<'a, T> {
	Owned(Vec<T>),
	#[cfg_attr(feature = "serde", serde(skip_deserializing))]
	Borrowed(&'a [T]),
}

impl<'a, T: Clone> CoCow<'a, T> {
	/// Returns the inner owned value, cloning if it was borrowed.
	pub fn into_owned(self) -> T {
		match self {
			CoCow::Owned(owned) => owned,
			CoCow::Borrowed(borrowed) => borrowed.clone(),
		}
	}
	/// Returns a mutable reference to the inner owned value, cloning if it was borrowed.
	pub fn to_mut(&mut self) -> &mut T {
		match self {
			CoCow::Owned(owned) => owned,
			CoCow::Borrowed(borrowed) => {
				*self = CoCow::Owned(borrowed.clone());
				match self {
					CoCow::Owned(owned) => owned,
					_ => unreachable!(),
				}
			}
		}
	}
}
impl<'a, T> CoCow<'a, T> {
	/// Returns `true` if the value is borrowed.
	pub fn is_borrowed(&self) -> bool {
		matches!(self, CoCow::Borrowed(_))
	}
	/// Returns `true` if the value is owned.
	pub fn is_owned(&self) -> bool {
		matches!(self, CoCow::Owned(_))
	}
}

impl<'a, T: Clone> CoCowSlice<'a, T> {
	/// Returns the inner owned [`Vec`][std::vec::Vec], cloning if it was borrowed.
	pub fn into_owned(self) -> Vec<T> {
		match self {
			CoCowSlice::Owned(owned) => owned,
			CoCowSlice::Borrowed(borrowed) => borrowed.to_owned(),
		}
	}
	/// Returns a mutable reference to the inner owned [`Vec`][std::vec::Vec], cloning if it was borrowed.
	pub fn to_mut(&mut self) -> &mut Vec<T> {
		match self {
			CoCowSlice::Owned(owned) => owned,
			CoCowSlice::Borrowed(borrowed) => {
				*self = CoCowSlice::Owned(borrowed.to_owned());
				match self {
					CoCowSlice::Owned(owned) => owned,
					_ => unreachable!(),
				}
			}
		}
	}
}
impl<'a, T> CoCowSlice<'a, T> {
	/// Returns `true` if the value is borrowed.
	pub fn is_borrowed(&self) -> bool {
		matches!(self, CoCowSlice::Borrowed(_))
	}
	/// Returns `true` if the value is owned.
	pub fn is_owned(&self) -> bool {
		matches!(self, CoCowSlice::Owned(_))
	}
}

impl<'a, T> ShallowClone<'a> for CoCow<'a, T> {
	type Target = CoCow<'a, T>;

	fn shallow_clone(&'a self) -> Self::Target {
		match self {
			CoCow::Owned(owned) => CoCow::Borrowed(&owned),
			CoCow::Borrowed(borrowed) => CoCow::Borrowed(borrowed),
		}
	}
}
impl<'a, T> ShallowClone<'a> for CoCowSlice<'a, T> {
	type Target = CoCowSlice<'a, T>;

	fn shallow_clone(&'a self) -> Self::Target {
		match self {
			CoCowSlice::Owned(owned) => CoCowSlice::Borrowed(&owned),
			CoCowSlice::Borrowed(borrowed) => CoCowSlice::Borrowed(borrowed),
		}
	}
}

impl<'a, T: MakeOwned> MakeOwned for CoCow<'a, T>
where
	<T as MakeOwned>::Owned: Clone,
{
	type Owned = CoCow<'static, <T as MakeOwned>::Owned>;

	fn make_owned(self) -> <Self as MakeOwned>::Owned {
		CoCow::Owned(self.into_owned().make_owned())
	}
}
impl<'a, T: MakeOwned> MakeOwned for CoCowSlice<'a, T>
where
	<T as MakeOwned>::Owned: Clone,
{
	type Owned = CoCowSlice<'static, <T as MakeOwned>::Owned>;

	fn make_owned(self) -> <Self as MakeOwned>::Owned {
		CoCowSlice::Owned(self.into_owned().make_owned())
	}
}

impl<'a, T> Deref for CoCow<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self {
			CoCow::Owned(owned) => owned,
			CoCow::Borrowed(borrowed) => borrowed,
		}
	}
}
impl<'a, T> Deref for CoCowSlice<'a, T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		match self {
			CoCowSlice::Owned(owned) => owned,
			CoCowSlice::Borrowed(borrowed) => borrowed,
		}
	}
}

impl<'a, T> AsRef<T> for CoCow<'a, T> {
	fn as_ref(&self) -> &T {
		self
	}
}
impl<'a, T> AsRef<[T]> for CoCowSlice<'a, T> {
	fn as_ref(&self) -> &[T] {
		self
	}
}

impl<'a, T> Borrow<T> for CoCow<'a, T> {
	fn borrow(&self) -> &T {
		self
	}
}
impl<'a, T> Borrow<[T]> for CoCowSlice<'a, T> {
	fn borrow(&self) -> &[T] {
		self
	}
}

impl<'a, T: Default> Default for CoCow<'a, T> {
	fn default() -> Self {
		CoCow::Owned(Default::default())
	}
}
impl<'a, T> Default for CoCowSlice<'a, T> {
	fn default() -> Self {
		CoCowSlice::Owned(Default::default())
	}
}

impl<'a, T: Display> Display for CoCow<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		Display::fmt(&**self, f)
	}
}

impl<'a, T> From<T> for CoCow<'a, T> {
	fn from(value: T) -> Self {
		CoCow::Owned(value)
	}
}
impl<'a, T> From<Vec<T>> for CoCowSlice<'a, T> {
	fn from(value: Vec<T>) -> Self {
		CoCowSlice::Owned(value)
	}
}

impl<'a, T> From<&'a T> for CoCow<'a, T> {
	fn from(value: &'a T) -> Self {
		CoCow::Borrowed(value)
	}
}
impl<'a, T> From<&'a [T]> for CoCowSlice<'a, T> {
	fn from(value: &'a [T]) -> Self {
		CoCowSlice::Borrowed(value)
	}
}
impl<'a, T> From<&'a Vec<T>> for CoCowSlice<'a, T> {
	fn from(value: &'a Vec<T>) -> Self {
		CoCowSlice::Borrowed(value)
	}
}
impl<'a, const N: usize, T> From<&'a [T; N]> for CoCowSlice<'a, T> {
	fn from(value: &'a [T; N]) -> Self {
		CoCowSlice::Borrowed(value)
	}
}

impl<'a, T: Clone> From<Cow<'a, T>> for CoCow<'a, T> {
	fn from(value: Cow<'a, T>) -> Self {
		match value {
			Cow::Borrowed(borrowed) => Self::Borrowed(borrowed),
			Cow::Owned(owned) => Self::Owned(owned),
		}
	}
}
impl<'a, T: Clone> From<Cow<'a, [T]>> for CoCowSlice<'a, T> {
	fn from(value: Cow<'a, [T]>) -> Self {
		match value {
			Cow::Borrowed(borrowed) => Self::Borrowed(borrowed),
			Cow::Owned(owned) => Self::Owned(owned),
		}
	}
}

impl<'a, T> IntoIterator for &'a CoCow<'a, T>
where
	&'a T: IntoIterator,
{
	type Item = <&'a T as IntoIterator>::Item;
	type IntoIter = <&'a T as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			CoCow::Owned(owned) => owned.into_iter(),
			CoCow::Borrowed(borrowed) => borrowed.into_iter(),
		}
	}
}
impl<'a, T> IntoIterator for &'a CoCowSlice<'a, T>
where
	&'a [T]: IntoIterator,
{
	type Item = <&'a [T] as IntoIterator>::Item;
	type IntoIter = <&'a [T] as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			CoCowSlice::Owned(owned) => (&owned[..]).into_iter(),
			CoCowSlice::Borrowed(borrowed) => borrowed.into_iter(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{CoCow, CoCowSlice};
	use crate::ShallowClone;

	#[test]
	fn test_covariance() {
		// make sure these cows are actually covariant
		#[derive(ShallowClone, Clone)]
		struct MyStruct<'a>(#[allow(dead_code)] &'a ());

		let u = ();

		let x = MyStruct(&u);
		let cocow: CoCow<MyStruct> = CoCow::from(x);
		fn test<'a>(_: CoCow<'a, MyStruct<'a>>) {}
		test(cocow.shallow_clone());

		let y = [(); 100].map(|_| MyStruct(&u));
		let cocow_slice: CoCowSlice<MyStruct> = CoCowSlice::from(&y[..]);
		fn test_slice<'a>(_: CoCowSlice<'a, MyStruct<'a>>) {}
		test_slice(cocow_slice.shallow_clone());
	}
}
