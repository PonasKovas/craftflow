#![doc(
	html_favicon_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
#![doc(
	html_logo_url = "https://github.com/PonasKovas/craftflow/blob/master/assets/icon.png?raw=true"
)]
// used when generating events with a macro for abstract packets
// (direction_macro.rs)
#![feature(macro_metavar_expr_concat)]
// also in direction_macro.rs for various stuff
#![feature(macro_metavar_expr)]

pub use craftflow_protocol_versions::{MAX_VERSION, MIN_VERSION};

pub mod c2s;
pub(crate) mod packet_constructor;
mod packet_new;
mod packet_write;
pub mod s2c;

pub use c2s::AbC2S;
pub use packet_constructor::*;
pub use packet_new::*;
pub use packet_write::*;
pub use s2c::AbS2C;

/// Contains all the possible states of a connection
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum State {
	Handshake,
	Status,
	Login,
	Configuration,
	Play,
}

/// Returned by an abstract packet constructor to indicate the result of processing a packet
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ConstructorResult<D, C> {
	/// The constructor is done and the abstract packet is ready
	Done(D),
	/// The constructor needs more packets to finish
	Continue(C),
	/// The packet was not of use for this constructor
	Ignore,
}

/// Returned by an abstract packet writer to indicate whether the packet can be written for
/// the given protocol version
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum WriteResult<T> {
	/// The writer successfully converted the abstract packet to concrete packets
	Success(T),
	/// This abstract packet has no implementation for the given protocol version or state
	Unsupported,
}

impl<D, C> ConstructorResult<D, C> {
	/// Maps the inner value of the `Continue` variant
	pub fn map_continue<T>(self, f: impl FnOnce(C) -> T) -> ConstructorResult<D, T> {
		match self {
			Self::Done(d) => ConstructorResult::Done(d),
			Self::Continue(c) => ConstructorResult::Continue(f(c)),
			Self::Ignore => ConstructorResult::Ignore,
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
	pub fn assume_success(self) -> T {
		match self {
			Self::Success(inner) => inner,
			_ => panic!("WriteResult::assume: not Success"),
		}
	}
}
