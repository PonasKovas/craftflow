use thiserror::Error;

pub(crate) const LIMIT: usize = u16::MAX as usize;

#[derive(Error, Debug)]
#[error("length of {LIMIT} exceeded")]
pub struct LengthExceeded;

/// NBT String - a wrapper around normal [`String`] but enforces a length limit
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct NbtString {
	s: String,
}

/// NBT Str - a wrapper around normal [`str`] but enforces a length limit
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(transparent)]
pub struct NbtStr {
	s: str,
}

fn calc_mcesu8_len(s: &str) -> usize {
	let mut extra = 0;
	for c in s.chars() {
		if c == '\u{0}' {
			extra += 1; // NUL is represented as \xC0 \x80
		}
		if c > '\u{FFFF}' {
			extra += 2; // each 4-byte UTF-8 sequence (BMP > U+FFFF) becomes 6 bytes in CESU-8 (2 extra bytes per character).
		}
	}

	s.len() + extra
}

impl NbtStr {
	pub unsafe fn new_unchecked(s: &str) -> &Self {
		unsafe { std::mem::transmute(s) }
	}
	pub fn from_str(s: &str) -> Result<&Self, LengthExceeded> {
		if calc_mcesu8_len(&s) > LIMIT {
			return Err(LengthExceeded);
		}

		Ok(unsafe { Self::new_unchecked(s) })
	}
}

impl std::ops::Deref for NbtStr {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.s
	}
}

impl NbtString {
	pub unsafe fn new_unchecked(s: String) -> Self {
		Self { s }
	}
	/// Converts a normal [`str`] to [`NbtString`].
	///
	/// Will fail if length (in bytes) exceeds `65535`.
	pub fn from_str(s: &str) -> Result<Self, LengthExceeded> {
		if calc_mcesu8_len(s) > LIMIT {
			return Err(LengthExceeded);
		}

		Ok(Self { s: s.to_owned() })
	}
	/// Converts (cheap) a normal String to [`NbtString`].
	///
	/// Will fail if length (in bytes) exceeds `65535`.
	pub fn from_string(s: String) -> Result<Self, LengthExceeded> {
		if calc_mcesu8_len(&s) > LIMIT {
			return Err(LengthExceeded);
		}

		Ok(Self { s })
	}
	/// Gives the inner String.
	pub fn into_inner(self) -> String {
		self.s
	}
	/// Gives an immutable reference to the inner String.
	pub fn as_string(&self) -> &String {
		&self.s
	}
	/// Truncates this [`NbtString`], removing all contents.
	///
	/// See [`String::clear`] for more information.
	pub fn clear(&mut self) {
		self.s.clear();
	}
	/// Removes the specified range from the string in bulk, returning all removed characters as an iterator.
	///
	/// See [`String::drain`] for more information.
	pub fn drain<R>(&mut self, range: R)
	where
		R: std::ops::RangeBounds<usize>,
	{
		self.s.drain(range);
	}
	/// Creates a new empty [`NbtString`].
	///
	/// See [`String::new`] for more information.
	pub fn new() -> Self {
		Self { s: String::new() }
	}
	/// Removes the last character from the string buffer and returns it.
	///
	/// See [`String::pop`] for more information.
	pub fn pop(&mut self) -> Option<char> {
		self.s.pop()
	}
	/// Removes a [`char`] from this [`NbtString`] at a byte position and returns it.
	///
	/// See [`String::remove`] for more information.
	pub fn remove(&mut self, idx: usize) -> char {
		self.s.remove(idx)
	}
	/// Retains only the characters specified by the predicate.
	///
	/// See [`String::retain`] for more information.
	pub fn retain<F>(&mut self, f: F)
	where
		F: FnMut(char) -> bool,
	{
		self.s.retain(f);
	}
	/// Shrinks the capacity of this [`NbtString`] with a lower bound.
	///
	/// See [`String::shrink_to`] for more information.
	pub fn shrink_to(&mut self, min_capacity: usize) {
		self.s.shrink_to(min_capacity)
	}
	/// Shrinks the capacity of this [`NbtString`] to match its length.
	///
	/// See [`String::shrink_to_fit`] for more information.
	pub fn shrink_to_fit(&mut self) {
		self.s.shrink_to_fit()
	}
	/// Splits the string into two at the given byte index.
	///
	/// See [`String::split_off`] for more information.
	pub fn split_off(&mut self, at: usize) -> NbtString {
		Self {
			s: self.s.split_off(at),
		}
	}
	/// Shortens this [`NbtString`] to the specified length.
	///
	/// See [`String::truncate`] for more information.
	pub fn truncate(&mut self, new_len: usize) {
		self.s.truncate(new_len);
	}
}

impl std::ops::Deref for NbtString {
	type Target = NbtStr;

	fn deref(&self) -> &Self::Target {
		// Safety: NbtString and NbtStr have the same invariants
		unsafe { NbtStr::new_unchecked(&self.s) }
	}
}

impl AsRef<std::ffi::OsStr> for NbtString {
	fn as_ref(&self) -> &std::ffi::OsStr {
		self.s.as_ref()
	}
}
impl AsRef<std::path::Path> for NbtString {
	fn as_ref(&self) -> &std::path::Path {
		self.s.as_ref()
	}
}
impl AsRef<[u8]> for NbtString {
	fn as_ref(&self) -> &[u8] {
		self.s.as_ref()
	}
}
impl AsRef<str> for NbtString {
	fn as_ref(&self) -> &str {
		self.s.as_ref()
	}
}

impl std::borrow::Borrow<str> for NbtString {
	fn borrow(&self) -> &str {
		self.s.borrow()
	}
}

impl std::fmt::Display for NbtString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.s.fmt(f)
	}
}

impl<'a> From<&'a NbtString> for std::borrow::Cow<'a, str> {
	fn from(value: &'a NbtString) -> Self {
		std::borrow::Cow::Borrowed(value)
	}
}
impl From<&NbtString> for NbtString {
	fn from(value: &NbtString) -> Self {
		value.clone()
	}
}
impl From<NbtString> for std::sync::Arc<str> {
	fn from(value: NbtString) -> Self {
		std::sync::Arc::from(value.s)
	}
}
impl<'a> From<NbtString> for Box<dyn std::error::Error + 'a> {
	fn from(value: NbtString) -> Self {
		Box::from(value.s)
	}
}
impl<'a> From<NbtString> for Box<dyn std::error::Error + Sync + Send + 'a> {
	fn from(value: NbtString) -> Self {
		Box::from(value.s)
	}
}
impl From<NbtString> for Box<str> {
	fn from(value: NbtString) -> Self {
		Box::from(value.s)
	}
}
impl From<NbtString> for std::ffi::OsString {
	fn from(value: NbtString) -> Self {
		std::ffi::OsString::from(value.s)
	}
}
impl From<NbtString> for std::path::PathBuf {
	fn from(value: NbtString) -> Self {
		std::path::PathBuf::from(value.s)
	}
}
impl From<NbtString> for std::rc::Rc<str> {
	fn from(value: NbtString) -> Self {
		std::rc::Rc::from(value.s)
	}
}
impl From<NbtString> for Vec<u8> {
	fn from(value: NbtString) -> Self {
		Vec::from(value.s)
	}
}

impl<I> std::ops::Index<I> for NbtString
where
	I: std::slice::SliceIndex<str>,
{
	type Output = I::Output;

	fn index(&self, index: I) -> &Self::Output {
		self.s.index(index)
	}
}

impl<'a> PartialEq<&'a str> for NbtString {
	fn eq(&self, other: &&'a str) -> bool {
		self.s.eq(other)
	}
}
impl<'a> PartialEq<std::borrow::Cow<'a, str>> for NbtString {
	fn eq(&self, other: &std::borrow::Cow<'a, str>) -> bool {
		self.s.eq(other)
	}
}
impl<'a> PartialEq<NbtString> for &'a str {
	fn eq(&self, other: &NbtString) -> bool {
		self.eq(&other.s)
	}
}
impl PartialEq<NbtString> for str {
	fn eq(&self, other: &NbtString) -> bool {
		self.eq(&other.s)
	}
}
impl PartialEq<str> for NbtString {
	fn eq(&self, other: &str) -> bool {
		self.s.eq(other)
	}
}

impl std::net::ToSocketAddrs for NbtString {
	type Iter = <String as std::net::ToSocketAddrs>::Iter;

	fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
		self.s.to_socket_addrs()
	}
}

impl TryFrom<String> for NbtString {
	type Error = LengthExceeded;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::from_string(value)
	}
}
impl TryFrom<&str> for NbtString {
	type Error = LengthExceeded;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::from_str(value)
	}
}

impl std::str::FromStr for NbtString {
	type Err = LengthExceeded;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if calc_mcesu8_len(&s) > LIMIT {
			return Err(LengthExceeded);
		}

		Ok(Self { s: s.to_owned() })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_calc_mcesu8_len() {
		// 1. Empty string
		assert_eq!(calc_mcesu8_len(""), 0);

		// 2. ASCII characters (1 byte in UTF-8, no change in CESU-8)
		assert_eq!(calc_mcesu8_len("a"), 1);
		assert_eq!(calc_mcesu8_len("abc"), 3);

		// 3. NUL character (U+0000)
		let s = "\u{0}"; // UTF-8: 1 byte → CESU-8: 2 bytes (+1)
		assert_eq!(calc_mcesu8_len(s), 2);

		// 4. BMP character (3-byte UTF-8, no change in CESU-8)
		let s = "\u{0800}"; // UTF-8: 3 bytes → CESU-8: 3 bytes (+0)
		assert_eq!(calc_mcesu8_len(s), 3);

		// 5. Supplementary character (U+1F600 = 😀)
		let s = "\u{1F600}"; // UTF-8: 4 bytes → CESU-8: 6 bytes (+2)
		assert_eq!(calc_mcesu8_len(s), 6);

		// 6. Mixed cases
		let s = "a\u{0}b\u{1F600}c";
		// UTF-8: 1(a) + 1(NUL) + 1(b) + 4(😀) + 1(c) = 8 bytes
		// CESU-8: 1(a) + 2(NUL) + 1(b) + 6(😀) + 1(c) = 11 bytes
		assert_eq!(calc_mcesu8_len(s), 11);

		// 7. Edge case: Maximum supplementary character (U+10FFFF)
		let s = "\u{10FFFF}"; // UTF-8: 4 bytes → CESU-8: 6 bytes (+2)
		assert_eq!(calc_mcesu8_len(s), 6);

		// 8. String with multiple NULs and supplementaries
		let s = "\u{0}\u{1F600}\u{0}";
		// UTF-8: 1 + 4 + 1 = 6 bytes
		// CESU-8: 2 + 6 + 2 = 10 bytes
		assert_eq!(calc_mcesu8_len(s), 10);
	}
}
