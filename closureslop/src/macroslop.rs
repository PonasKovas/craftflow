/// Registers a callback to a reactor instance.
///
/// This is a more powerful method, but in most cases you should use the combination of [`#[callback]`][crate::callback]
/// and [`reg!`][crate::reg] macros. This macro allows to give a closure as the callback instead of a full function,
/// and also allows setting a custom name.
///
/// # Usage
///
/// ```ignore
/// add_callback!(
/// 	reactor, // the reactor instance to which to register the callback to
/// 	EventType => "callback_name" => |ctx, args| SmallBox::new(async move { ... }),
/// 	<after: "crate:callback">, // arbitrary number of ordering requests
/// 	<before: "crate:callback">,
/// );
/// ```
///
/// For `before` and `after` arguments, the callback IDs are in the format of `"defining_crate_name:callback_name"`.
/// If registered using the [`#[callback]`][crate::callback] attribute macro the name will be the same as the function name.
///
/// If you specify ordering requests relative to a callback that is not found at runtime, the ordering request will
/// be just silently ignored. If you want to rely on another callback being present, use other methods. These are just
/// ordering requests, not dependency declarations.
///
/// # Example
///
/// ```
/// # use closureslop::{add_callback, Event, Reactor};
/// # use std::ops::ControlFlow;
/// # use smallbox::SmallBox;
/// # struct MyEvent;
/// # impl Event for MyEvent { type Args<'a> = &'a str; type Return = (); }
/// # let mut reactor: Reactor<()> = Reactor::new();
/// add_callback!(reactor, MyEvent => "my_callback" => |ctx: &(), args: &mut &str| SmallBox::new(async move {
///		// your code here
///		ControlFlow::Continue(())
/// }), before: "another_crate:another_callback");
/// ```
#[macro_export]
macro_rules! add_callback {
	($reactor:expr, $event:ty => $name:expr => $callback:expr $(, $($order:tt)* )?) => {
		$crate::__internal_add_callback!($reactor, $event => $name => $callback $(, $($order)* )?);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_add_callback {
	($reactor:expr, $event:ty => $name:expr => $callback:expr $(, $($order:tt)* )?) => {
		#[allow(unused_mut)]
		let mut after = Vec::new();
		#[allow(unused_mut)]
		let mut before = Vec::new();

		$(
			$crate::__internal_add_callback!(@order: after, before => [ $($order)* ]);
		)?

		$reactor.add_callback::<$event, _>(
			format!("{}:{}", env!("CARGO_CRATE_NAME"), $name),
			after,
			before,
			$callback,
		);
	};
	(@order: $after:ident, $before:ident => []) => {};
	(@order: $after:ident, $before:ident => [after: $target:expr $(, $($order:tt)* )?]) => {
		$after.push($target.to_string());
		$(
			$crate::__internal_add_callback!(@order: $after, $before => [ $($order)* ]);
		)?
	};
	(@order: $after:ident, $before:ident => [before: $target:expr $(, $($order:tt)* )?]) => {
		$before.push($target.to_string());
		$(
			$crate::__internal_add_callback!(@order: $after, $before => [ $($order)* ]);
		)?
	};
}

// re-exports used in closureslop-macros proc macros expansions

pub use linkme;
pub use smallbox;
