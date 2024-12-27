//! Closureslop - a simple asynchronous callback system.
//!
//! This library provides a simple, type-safe, and asynchronous callback system, allowing you
//! to create events, register multiple handlers for them and trigger them.
//!

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
/// - `group` - **optional** identifier for the group of callbacks to add it to. Must have a respective [`init!`][crate::init].
/// - `event` - the type of the event for which this callback is.
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
/// ```rust
/// # use closureslop::{callback, Event, init};
/// # use std::ops::ControlFlow;
/// # struct MyEvent;
/// # impl Event for MyEvent { type Args<'a> = &'a str; type Return = (); }
/// init!(ctx (), id: "optional_events"); // must be at the root of your crate (lib.rs or main.rs)
///
/// #[callback(group: "optional_events", event: MyEvent, before: "another_crate:another_callback")]
/// async fn name_of_the_callback(ctx: &(), args: &mut &str) -> ControlFlow<()> {
///    // your code here
/// }
/// ```
pub use closureslop_macros::callback;
/// Initializes a "collector" for callbacks in this crate.
///
/// **Must be at the root of the crate.** (not in any function or module, just straight up root-level)
///
/// Once you add this to your crate, you can annotate functions anywhere in your crate with the
/// [`#[callback]`][crate::callback] attribute and then register them to a reactor instance using
/// the [`reg!`][crate::reg] macro.
///
/// # Arguments
///
/// - `ctx` - reactor context type. The `T` in `Reactor<T>` that you will want these events to be for.
/// - `id` - **optional** identifier for a group of events. To register events in the same group, use the same `id`.
///
/// # Example
///
/// ```rust
/// # use closureslop::init;
/// # struct MyContext;
/// init!(ctx: ()); // no id, default event group
/// init!(ctx: MyContext, id: "optional_events");
/// ```
pub use closureslop_macros::init;
/// Registers a group of callbacks to a reactor instance.
///
/// # Arguments
///
/// - `id` - **optional** identifier for a group of events to register. Must have a respective [`init!`][crate::init].
/// - `to` - the reactor instance to which to register the callbacks.
///
/// # Example
///
/// ```rust
/// # use closureslop::{init, reg, Reactor};
/// init!(ctx: ()); // default group
/// init!(ctx: (), id: "specific"); // another named group
///
/// fn main() {
///     let mut reactor = Reactor::new();
///     reg!(to: &mut reactor);
///     reg!(id: "specific", to: &mut reactor);
/// }
/// ```
pub use closureslop_macros::reg;

pub use event::Event;
pub use reactor::Reactor;

/// The stack size of the smallboxes of futures in async closures.
#[doc(hidden)]
type _SmallBoxSize = [usize; 4];
