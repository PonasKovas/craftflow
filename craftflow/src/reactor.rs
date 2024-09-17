//! The reactor is a structure that allows to register functions that will run on specific events
//!

use std::{
	any::{Any, TypeId},
	collections::BTreeMap,
	marker::PhantomData,
	ops::ControlFlow,
};

/// Marks an event type
///
/// Given the implementation of this trait, the handlers that the reactor will use for this event
/// will be
/// ```ignore
/// Fn(&CTX, Event::Args) -> ControlFlow<Event::Return, Event::Args>
/// ```
/// Return `ControlFlow::Continue(Event::Args)` to continue reacting to the event with the next registered
/// handler, or `ControlFlow::Break(Event::Return)` to stop the event and return.
pub trait Event: Any {
	/// The type of the arguments that the event will receive
	type Args;
	/// The type of the return value of the event
	type Return;
}

/// The reactor structure allows to register functions that will run on specific events
/// and then trigger the events
///
/// The reactor is generic over the context type `CTX`, which is the type of the context that will be
/// passed to the event handlers
pub struct Reactor<CTX> {
	// The `dyn Any` is actually a type erased `Box<dyn Fn(&CTX, Event::Args) -> ControlFlow<Event::Return, Event::Args>>`
	// But we can't store it directly because Event is different for each event type
	events: BTreeMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
	// events: BTreeMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
	_phantom: PhantomData<fn(&CTX)>,
}

// These are not automatically implemented because the Box<dyn Any> might be not Sync + Send
// We can't mark it as Sync + Send there because then we can't downcast it
// (downcast is only implemented for dyn Any, not dyn Any + Sync + Send)
// But in reality we know that all event handlers are Sync + Send, because we have
// bound checks for it in the add_handler method
// unsafe impl<CTX> Sync for Reactor<CTX> {}
// unsafe impl<CTX> Send for Reactor<CTX> {}

impl<CTX: 'static> Reactor<CTX> {
	/// Create a new empty reactor
	pub fn new() -> Self {
		Self {
			events: BTreeMap::new(),
			_phantom: PhantomData,
		}
	}
	/// Register a handler for an event
	pub fn add_handler<
		E: Event,
		F: Fn(&CTX, E::Args) -> ControlFlow<E::Return, E::Args> + Sync + Send + 'static,
	>(
		&mut self,
		handler: F,
	) {
		let pos = self
			.events
			.get(&TypeId::of::<E>())
			.map(|handlers| handlers.len())
			.unwrap_or(0);

		self.add_handler_at_pos::<E, _>(pos, handler);
	}
	/// Register a handler for an event at a specific position between the existing handlers
	/// If the position is greater than the number of handlers, the handler will be added at the end
	pub fn add_handler_at_pos<
		E: Event,
		F: Fn(&CTX, E::Args) -> ControlFlow<E::Return, E::Args> + Sync + Send + 'static,
	>(
		&mut self,
		pos: usize,
		handler: F,
	) {
		let closure = Box::new(handler)
			as Box<
				dyn Fn(&CTX, E::Args) -> ControlFlow<E::Return, E::Args> + Send + Sync + 'static,
			>;

		// // Erase the type of the closure so we can store it
		let type_erased = Box::new(closure) as Box<dyn Any + Send + Sync + 'static>;

		let handlers = self.events.entry(TypeId::of::<E>()).or_insert(Vec::new());

		// clamp the pos to valid range
		let pos = pos.min(handlers.len());

		handlers.insert(pos, type_erased);
	}
	/// Trigger an event
	pub fn event<E: Event>(&self, ctx: &CTX, mut args: E::Args) -> ControlFlow<E::Return, E::Args> {
		if let Some(handlers) = self.events.get(&TypeId::of::<E>()) {
			for handler in handlers {
				// Convert back to the real closure type
				let closure: &Box<
					dyn Fn(&CTX, E::Args) -> ControlFlow<E::Return, E::Args> + Send + Sync,
				> = handler.downcast_ref().unwrap();

				args = closure(ctx, args)?;
			}
		}

		ControlFlow::Continue(args)
	}
}

impl<CTX> std::fmt::Debug for Reactor<CTX> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Reactor")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_reactor() {
		struct MyEvent;
		impl Event for MyEvent {
			type Args = u32;
			type Return = ();
		}

		struct MyEvent2;
		impl Event for MyEvent2 {
			type Args = ();
			type Return = &'static str;
		}

		let mut reactor = Reactor::<()>::new();

		reactor.add_handler_at_pos::<MyEvent, _>(999, |_ctx, arg| {
			println!("First handler: {}", arg);

			ControlFlow::Continue(arg)
		});
		reactor.add_handler_at_pos::<MyEvent, _>(0, |_ctx, mut arg| {
			println!("Second handler: {}", arg);

			arg *= 2;

			ControlFlow::Continue(arg)
		});

		reactor.add_handler::<MyEvent2, _>(|_ctx, ()| {
			println!("first MyEvent2");

			ControlFlow::Continue(())
		});
		reactor.add_handler_at_pos::<MyEvent2, _>(1, |_ctx, ()| {
			println!("second MyEvent2");

			ControlFlow::Break("test")
		});
		reactor.add_handler_at_pos::<MyEvent2, _>(2, |_ctx, ()| {
			println!("third MyEvent2");

			ControlFlow::Break("this should not be reached")
		});

		assert_eq!(reactor.event::<MyEvent>(&(), 7), ControlFlow::Continue(14));
		assert_eq!(
			reactor.event::<MyEvent2>(&(), ()),
			ControlFlow::Break("test")
		);
	}
}
