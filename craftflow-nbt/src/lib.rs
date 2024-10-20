pub mod arrays;
mod error;
pub(crate) mod ser;
pub(crate) mod tag;

pub use error::{Error, Result};
pub use ser::{to_writer, to_writer_named};
