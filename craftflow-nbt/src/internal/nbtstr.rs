pub use maxlen::bstr;

/// Creates a static `NbtStr`
///
/// ```
/// # use craftflow_nbt::{nbtstr, NbtStr};
/// const S: &'static NbtStr = nbtstr!("hello");
/// ```
#[macro_export]
macro_rules! nbtstr {
	($str:expr) => {
		$crate::internal::nbtstr::bstr!(65535, MCesu8, $str)
	};
}
