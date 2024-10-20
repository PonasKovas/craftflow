pub mod arrays;
mod error;
pub(crate) mod ser;
pub(crate) mod tag;

pub use error::{Error, Result};
pub use ser::{serialize, serialize_named};
