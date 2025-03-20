use std::marker::PhantomData;

/// Mmmm 🤤😋
pub struct PacketEat<T, P>(PhantomData<fn(T, P) -> (T, P)>);

impl<T: Into<P>, P> PacketEat<T, P> {
	pub(crate) fn new() -> Self {
		Self(PhantomData)
	}

	pub fn feed(self, packet: T) -> P {
		packet.into()
	}
}
