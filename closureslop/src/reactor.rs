mod callbacks;

use crate::{_SmallBoxSize, Event};
use callbacks::{Callback, Callbacks};
use smallbox::SmallBox;
use std::{
	any::{Any, TypeId},
	collections::BTreeMap,
	marker::PhantomData,
	ops::ControlFlow,
};

/// The reactor structure allows to register functions that will run on specific events
/// and then trigger the events
///
/// The reactor is generic over the context type `CTX`, which is the type of the context that will be
/// passed to the event handlers
pub struct Reactor<CTX> {
	// The `dyn Any` is actually a type erased `Box<dyn Fn(...) -> ...`
	// But we can't store it directly because Event is different for each event type
	events: BTreeMap<TypeId, Callbacks>,
	_phantom: PhantomData<fn(CTX)>,
}

impl<CTX: 'static> Reactor<CTX> {
	/// Create a new empty reactor
	pub fn new() -> Self {
		Self {
			events: BTreeMap::new(),
			_phantom: PhantomData,
		}
	}
	/// Register a callback for an event. Prefer using macros instead.
	///
	/// Panics if there are cyclic dependencies detected in the callbacks (e.g. A must come after B, but B must come after A)
	///
	/// Panics if there's already a callback with the same id for this event.
	pub fn add_callback<
		E: Event,
		F: for<'a> Fn(
				&'a CTX,
				&'a mut E::Args<'_>,
			) -> SmallBox<
				dyn Future<Output = ControlFlow<E::Return>> + Send + 'a,
				_SmallBoxSize,
			> + Send
			+ Sync
			+ 'static,
	>(
		&mut self,
		id: String,
		must_come_after: Vec<String>,
		must_come_before: Vec<String>,
		handler: F,
	) {
		let closure = Box::new(handler)
			as Box<
				dyn for<'a> Fn(
						&'a CTX,
						&'a mut E::Args<'_>,
					) -> SmallBox<
						dyn Future<Output = ControlFlow<E::Return>> + Send + 'a,
						_SmallBoxSize,
					> + Send
					+ Sync
					+ 'static,
			>;
		let type_erased = Box::new(closure) as Box<dyn Any + Send + Sync + 'static>;

		let callbacks = self
			.events
			.entry(TypeId::of::<E>())
			.or_insert(Callbacks::new(std::any::type_name::<E>()));
		callbacks.add_callback(Callback {
			id,
			callback: type_erased,
			must_come_after,
			must_come_before,
		});
	}
	/// Trigger an event
	pub async fn trigger<E: Event>(
		&self,
		ctx: &CTX,
		args: &mut E::Args<'_>,
	) -> ControlFlow<E::Return> {
		if let Some(callbacks) = self.events.get(&TypeId::of::<E>()) {
			for callback in callbacks.in_order() {
				// Convert back to the real closure type
				let closure: &Box<
					dyn for<'a> Fn(
							&'a CTX,
							&'a mut E::Args<'_>,
						) -> SmallBox<
							dyn Future<Output = ControlFlow<E::Return>> + Send + 'a,
							_SmallBoxSize,
						> + Send
						+ Sync
						+ 'static,
				> = callback.callback.downcast_ref().unwrap();

				closure(ctx, args).await?;
			}
		}

		ControlFlow::Continue(())
	}
	/// Returns a nested list of all registered callbacks: `["event type" -> ["callback id"]]`
	pub fn list_callbacks(
		&self,
	) -> impl Iterator<Item = (&'static str, impl Iterator<Item = &str>)> {
		self.events
			.values()
			.map(|event| (event.event_name, event.in_order().map(|c| c.id.as_str())))
	}
}

impl<CTX: 'static> Default for Reactor<CTX> {
	fn default() -> Self {
		Self::new()
	}
}

impl<CTX> std::fmt::Debug for Reactor<CTX> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Closureslop Reactor")
	}
}
