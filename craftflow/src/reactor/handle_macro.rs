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
