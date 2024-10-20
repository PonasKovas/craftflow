#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

pub mod arrays;
mod error;
pub(crate) mod ser;
pub(crate) mod tag;

pub use error::{Error, Result};
pub use ser::{to_writer, to_writer_named};
