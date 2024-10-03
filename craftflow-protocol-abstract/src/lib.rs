#![feature(async_closure)]

//! It is intentional that the abstract packets are not grouped by the state of the connection.
//! They abstract the state too, since there is a history of adding new states to the protocol.
//!
//! It is still checked (dynamically) that the concrete packets (into which abstract ones are converted to)
//! are only used in the correct state.
//!
//! It is a responsibility of the user to ensure to use abstract packets in the correct state.

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
