use std::any::Any;

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
