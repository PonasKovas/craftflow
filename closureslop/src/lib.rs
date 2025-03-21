//! Closureslop - a simple asynchronous callback system.
//!
//! This library provides a simple, type-safe, and asynchronous callback system, allowing you
//! to create events, register multiple handlers for them and trigger them.
//!
//! # Usage
//!
//! ## Using attribute macros (convenient)
//!
//! ```
//! # use closureslop::{Event, Reactor, init, reg, callback};
//! # use std::ops::ControlFlow;
//!
//! struct MyEvent;
//! impl Event for MyEvent {
//! 	type Args<'a> = String;
//! 	type Return = u32;
//! }
//!
//! // "Collects" the #[callback] macros
//! init!(ctx: &'static str);
//!
//! #[callback(event: MyEvent)]
//! async fn my_callback(ctx: &&'static str, args: &mut String) -> ControlFlow<u32> {
//! 	println!("Callback called with context: {} and args: {}", ctx, args);
//! 	args.push_str(" world!");
//! 	ControlFlow::Continue(())
//! }
//!
//! # #[pollster::main]
//! async fn main() {
//! 	let mut reactor: Reactor<&'static str> = Reactor::new();
//! 	// register all collected callbacks (by init! and #[callback]) to this reactor instance
//! 	reg!(to: reactor);
//!
//! 	let ctx = "this will be available read-only to all callbacks in this reactor";
//! 	let mut args = String::from("hello");
//!
//! 	let result = reactor.trigger::<MyEvent>(&ctx, &mut args).await;
//! 	assert_eq!(result, ControlFlow::Continue(()));
//! 	assert_eq!(args, "hello world!");
//! }
//! ```
//!
//! ## Manually
//!
//! ```
//! use closureslop::{Event, Reactor, add_callback};
//! use std::ops::ControlFlow;
//! use smallbox::SmallBox;
//!
//! struct MyEvent;
//! impl Event for MyEvent {
//! 	type Args<'a> = String;
//! 	type Return = u32;
//! }
//!
//! # #[pollster::main]
//! async fn main() {
//! 	let mut reactor: Reactor<&'static str> = Reactor::new();
//!
//! 	add_callback!(reactor, MyEvent => "another_callback" => |ctx, arg| SmallBox::new(async move {
//! 		println!("manually added callback, not as convenient but more powerful");
//!			args.push_str(" world!");
//! 		ControlFlow::Break(7)
//! 	}));
//!
//! 	let ctx = "this will be available read-only to all callbacks in this reactor";
//! 	let mut args = String::from("hello");
//!
//! 	let result = reactor.trigger::<MyEvent>(&ctx, &mut args).await;
//! 	assert_eq!(result, ControlFlow::Break(7));
//! 	assert_eq!(args, "hello world!");
//! }
//! ```

/// macro related stuff
#[doc(hidden)]
#[path = "macroslop.rs"]
pub mod __private_macroslop;
mod event;
mod reactor;
#[cfg(test)]
mod tests;

/// Attribute macro for defining an event callback function.
///
/// Use this to mark your function and add it to a callback group defined by [`init!`][crate::init] in your crate root.
///
/// Then you can add these groups of callbacks to a reactor instance using the [`reg!`][crate::reg] macro.
///
/// # Arguments
///
/// - `event` - the type of the event for which this callback is.
/// - `group` - **optional** identifier for the group of callbacks to add it to. Must have a respective [`init!`][crate::init].
/// - `before` - **optional** callback id, that this callback must be executed before at runtime.
/// - `after` - **optional** callback id, that this callback must be executed after at runtime.
///
/// For `before` and `after` arguments, the callback IDs are in the format of `"defining_crate_name:function_name"` if
/// registered using this attribute macro. The [`add_callback!`][crate::add_callback] macro allows to set a custom name.
///
/// If you specify ordering requests relative to a callback that is not found at runtime, the ordering request will
/// be just silently ignored. If you want to rely on another callback being present, use other methods. These are just
/// ordering requests, not dependency declarations.
///
/// # Example
///
/// ```
/// # use closureslop::{callback, Event, init};
/// # use std::ops::ControlFlow;
/// # struct MyEvent;
/// # impl Event for MyEvent { type Args<'a> = &'a str; type Return = (); }
/// init!(ctx: (), group: "optional_events"); // must be at the root of your crate (lib.rs or main.rs)
///
/// #[callback(group: "optional_events", event: MyEvent, before: "another_crate:another_callback")]
/// async fn name_of_the_callback(ctx: &(), args: &mut &str) -> ControlFlow<()> {
///     // your code here
/// 	ControlFlow::Continue(())
/// }
///
/// # fn main() {}
/// ```
pub use closureslop_macros::callback;
/// Initializes a "collector" for callbacks in this crate.
///
/// **Must be at the root of the crate** (not in any function or module, just straight up root-level)
///
/// Once you add this to your crate, you can annotate functions anywhere in your crate with the
/// [`#[callback]`][crate::callback] attribute and then register them to a reactor instance using
/// the [`reg!`][crate::reg] macro.
///
/// # Arguments
///
/// - `ctx` - reactor context type. The `T` in `Reactor<T>` that you will want these events to be for.
/// - `group` - **optional** identifier for a group of events. To register events in the same group, use the same ID.
///
/// # Example
///
/// ```
/// # use closureslop::init;
/// # struct MyContext;
/// init!(ctx: ()); // no id, default event group
/// init!(ctx: MyContext, group: "optional_events");
/// ```
pub use closureslop_macros::init;
/// Registers a group of callbacks to a reactor instance.
///
/// # Arguments
///
/// - `group` - **optional** identifier for a group of events to register. Must have a respective [`init!`][crate::init].
/// - `to` - the reactor instance to which to register the callbacks.
///
/// # Example
///
/// ```
/// # use closureslop::{init, reg, Reactor};
/// init!(ctx: ()); // default group
/// init!(ctx: (), group: "specific"); // another named group
///
/// fn main() {
///     let mut reactor = Reactor::new();
///     reg!(to: &mut reactor);
///     reg!(group: "specific", to: reactor);
/// }
/// ```
pub use closureslop_macros::reg;

pub use event::Event;
pub use reactor::Reactor;

/// The stack size of the smallboxes of futures in async closures.
#[doc(hidden)]
type _SmallBoxSize = [usize; 4];
