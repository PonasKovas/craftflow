#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

pub mod arrays;
pub(crate) mod de;
pub mod dynamic;
mod error;
pub(crate) mod ser;
pub(crate) mod tag;
#[cfg(test)]
pub(crate) mod tests;

pub use de::{from_slice, from_slice_named};
pub use dynamic::DynNBT;
pub use error::{Error, Result};
pub use ser::{to_writer, to_writer_named};
