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
/// Fn(&CTX, Event::Args) -> ControlFlow<Event::Return>
/// ```
/// Return `ControlFlow::Continue(())` to continue reacting to the event with the next registered
/// handler, or `ControlFlow::Break(Event::Return)` to stop the event and return.
pub trait Event: Any {
	/// The type of the arguments that the event will receive
	type Args<'a>;
	/// The type of the return value of the event
	type Return;
}

/// The reactor structure allows to register functions that will run on specific events
/// and then trigger the events
///
/// The reactor is generic over the context type `CTX`, which is the type of the context that will be
/// passed to the event handlers
pub struct Reactor<CTX> {
	// The `dyn Any` is actually a type erased `Box<dyn Fn(&CTX, Event::Args) -> ControlFlow<Event::Return>>`
	// But we can't store it directly because Event is different for each event type
	events: BTreeMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
	_phantom: PhantomData<fn(&CTX)>,
}

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
		F: for<'a> Fn(&'a CTX, E::Args<'a>) -> ControlFlow<E::Return, E::Args<'a>>
			+ Sync
			+ Send
			+ 'static,
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
		F: for<'a> Fn(&'a CTX, E::Args<'a>) -> ControlFlow<E::Return, E::Args<'a>>
			+ Sync
			+ Send
			+ 'static,
	>(
		&mut self,
		pos: usize,
		handler: F,
	) {
		let closure = Box::new(handler)
			as Box<
				dyn for<'a> Fn(&'a CTX, E::Args<'a>) -> ControlFlow<E::Return, E::Args<'a>>
					+ Send
					+ Sync
					+ 'static,
			>;

		// // Erase the type of the closure so we can store it
		let type_erased = Box::new(closure) as Box<dyn Any + Send + Sync + 'static>;

		let handlers = self.events.entry(TypeId::of::<E>()).or_insert(Vec::new());

		// clamp the pos to valid range
		let pos = pos.min(handlers.len());

		handlers.insert(pos, type_erased);
	}
	/// Trigger an event
	pub fn event<'a, E: Event>(
		&self,
		ctx: &'a CTX,
		mut args: E::Args<'a>,
	) -> ControlFlow<E::Return, E::Args<'a>> {
		if let Some(handlers) = self.events.get(&TypeId::of::<E>()) {
			for handler in handlers {
				// Convert back to the real closure type
				let closure: &Box<
					dyn for<'b> Fn(&'b CTX, E::Args<'b>) -> ControlFlow<E::Return, E::Args<'b>>
						+ Send
						+ Sync,
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
			type Args<'a> = u32;
			type Return = ();
		}

		struct MyEvent2;
		impl Event for MyEvent2 {
			type Args<'a> = &'a str;
			type Return = String;
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

		reactor.add_handler::<MyEvent2, _>(|_ctx, a| {
			println!("first MyEvent2");

			ControlFlow::Continue(a)
		});
		reactor.add_handler_at_pos::<MyEvent2, _>(1, |_ctx, a| {
			println!("second MyEvent2");

			ControlFlow::Break(format!("{a}-test"))
		});
		reactor.add_handler_at_pos::<MyEvent2, _>(2, |_ctx, _a| {
			println!("third MyEvent2");

			ControlFlow::Break("this should not be reached".to_string())
		});

		assert_eq!(reactor.event::<MyEvent>(&(), 7), ControlFlow::Continue(14));
		assert_eq!(
			reactor.event::<MyEvent2>(&(), "my event2 test string"),
			ControlFlow::Break("my event2 test string-test".to_string())
		);
	}
}
