#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]

pub use craftflow_protocol_versions::{MAX_VERSION, MIN_VERSION};

pub mod c2s;
mod packet_constructor;
mod packet_new;
mod packet_write;
pub mod s2c;

pub use c2s::AbC2S;
pub use packet_constructor::*;
pub use packet_new::*;
pub use packet_write::*;
pub use s2c::AbS2C;

/// Returned by an abstract packet constructor to indicate the result of processing a packet
pub enum ConstructorResult<D, C, I> {
	/// The constructor is done and the abstract packet is ready
	Done(D),
	/// The constructor needs more packets to finish
	Continue(C),
	/// The packet was not of use for this constructor
	Ignore(I),
}

/// Returned by an abstract packet writer to indicate whether the packet can be written for
/// the given protocol version
pub enum WriteResult<T> {
	/// The writer successfully converted the abstract packet to concrete packets
	Success(T),
	/// This abstract packet has no implementation for the given protocol version
	Unsupported,
}

impl<D, C, I> ConstructorResult<D, C, I> {
	/// Maps the inner value of the `Continue` variant
	pub fn map_continue<T>(self, f: impl FnOnce(C) -> T) -> ConstructorResult<D, T, I> {
		match self {
			Self::Done(d) => ConstructorResult::Done(d),
			Self::Continue(c) => ConstructorResult::Continue(f(c)),
			Self::Ignore(i) => ConstructorResult::Ignore(i),
		}
	}
	/// Unwraps the `Done` variant, panicking if it's not `Done`
	pub fn assume_done(self) -> D {
		match self {
			Self::Done(d) => d,
			_ => panic!("ConstructorResult::assume_done: not Done"),
		}
	}
}

impl<T> WriteResult<T> {
	/// Unwraps the `Success` variant, panicking if it's `Unsupported`
	pub fn assume(self) -> T {
		match self {
			Self::Success(inner) => inner,
			_ => panic!("WriteResult::assume: not Success"),
		}
	}
}
