//! The reactor is a structure that allows to register functions that will run on specific events
//!

use smallbox::SmallBox;
use std::{
	any::{Any, TypeId},
	collections::BTreeMap,
	future::Future,
	marker::PhantomData,
	ops::ControlFlow,
};

/// Convenience macro for registering a handler for an event
///
/// # Usage
///
/// ```ignore
/// let reactor: Reactor<_> = ...;
/// handle!(reactor => MyEvent: {
///    println!("MyEvent handler");
///    ControlFlow::Continue(())
/// });
/// ```
#[macro_export]
macro_rules! handle {
	($reactor:expr => $event:ty: $ctx:pat, $arg:pat => $code:tt) => {
		$reactor
			.add_handler::<$event, _>(|$ctx, $arg| ::smallbox::SmallBox::new(async move $code ))
	};
}

// The stack size of the smallboxes.
type S = [usize; 4]; // 4 words

/// Marks an event type
///
/// Given the implementation of this trait, the handlers that the reactor will use for this event
/// will be
/// ```ignore
/// Fn(&CTX, &mut Event::Args) -> SmallBox<dyn Future<Output = ControlFlow<Event::Return>>>
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
	// The `dyn Any` is actually a type erased `Box<dyn Fn(...) -> ...`
	// But we can't store it directly because Event is different for each event type
	events: BTreeMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
	_phantom: PhantomData<fn(CTX) -> CTX>,
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
		F: for<'a, 'b> Fn(
				&'a CTX,
				&'a mut E::Args<'b>,
			) -> SmallBox<
				dyn Future<Output = ControlFlow<E::Return>> + Send + Sync + 'a,
				S,
			> + Sync
			+ Send
			+ 'static,
	>(
		&mut self,
		handler: F,
	) {
		let closure = Box::new(handler)
			as Box<
				dyn for<'a, 'b> Fn(
						&'a CTX,
						&'a mut E::Args<'b>,
					) -> SmallBox<
						dyn Future<Output = ControlFlow<E::Return>> + Send + Sync + 'a,
						S,
					> + Send
					+ Sync
					+ 'static,
			>;

		// Erase the type of the closure so we can store it
		let type_erased = Box::new(closure) as Box<dyn Any + Send + Sync + 'static>;

		let handlers = self.events.entry(TypeId::of::<E>()).or_insert(Vec::new());

		handlers.push(type_erased);
	}
	/// Trigger an event
	pub async fn event<E: Event>(
		&self,
		ctx: &CTX,
		args: &mut E::Args<'_>,
	) -> ControlFlow<E::Return> {
		if let Some(handlers) = self.events.get(&TypeId::of::<E>()) {
			for handler in handlers {
				// Convert back to the real closure type
				let closure: &Box<
					dyn for<'c, 'd> Fn(
							&'c CTX,
							&'c mut E::Args<'d>,
						) -> SmallBox<
							dyn Future<Output = ControlFlow<E::Return>> + Send + Sync + 'c,
							S,
						> + Send
						+ Sync
						+ 'static,
				> = handler.downcast_ref().unwrap();

				let fut = closure(ctx, args);
				fut.await?;
			}
		}

		ControlFlow::Continue(())
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

	#[tokio::test]
	async fn test_reactor() {
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

		handle!(reactor => MyEvent: _ctx, arg => {
			println!("First handler: {}", arg);

			ControlFlow::Continue(())
		});
		handle!(reactor => MyEvent: _ctx, arg => {
			println!("Second handler: {}", arg);

			*arg *= 2;

			ControlFlow::Continue(())
		});

		handle!(reactor => MyEvent2: _ctx, _ => {
			println!("first MyEvent2");

			ControlFlow::Continue(())
		});
		handle!(reactor => MyEvent2: _ctx, a => {
			println!("second MyEvent2");

			ControlFlow::Break(format!("{a}-test"))
		});
		handle!(reactor => MyEvent2: _ctx, _ => {
			println!("third MyEvent2");

			ControlFlow::Break("this should not be reached".to_string())
		});

		let mut x = 7;
		reactor
			.event::<MyEvent>(&(), &mut x)
			.await
			.continue_value()
			.unwrap();
		assert_eq!(x, 14);
		assert_eq!(
			reactor
				.event::<MyEvent2>(&(), &mut "my event2 test string")
				.await,
			ControlFlow::Break("my event2 test string-test".to_string())
		);
	}
}
