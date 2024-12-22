//! Closureslop - an asynchronous callback system.
//!
//! This library provides a simple, type-safe, and asynchronous callback system, allowing you
//! to create events, register multiple handlers for them and trigger them.
//!

mod event;
mod macroslop;
mod reactor;

pub use event::Event;
pub use reactor::Reactor;

#[cfg(test)]
mod tests {
	use smallbox::SmallBox;

	use super::*;
	use std::ops::ControlFlow;

	#[pollster::test]
	async fn simple() {
		let mut reactor = Reactor::<()>::new();

		struct MyEvent;
		impl Event for MyEvent {
			type Args<'a> = ();
			type Return = ();
		}

		add_callback!(reactor, MyEvent => "first" => |_ctx, _args| SmallBox::new(async move {
			println!("success yay");
			ControlFlow::Continue(())
		}));

		reactor.trigger::<MyEvent>(&(), &mut ()).await;
	}

	#[pollster::test]
	async fn change_arg() {
		let mut reactor = Reactor::<()>::new();

		struct MyEvent;
		impl Event for MyEvent {
			type Args<'a> = usize;
			type Return = ();
		}

		add_callback!(reactor, MyEvent => "lala" => |_ctx, args| SmallBox::new(async move {
			*args += 1;
			ControlFlow::Continue(())
		}));

		let mut x = 2;
		reactor.trigger::<MyEvent>(&(), &mut x).await;
		assert_eq!(x, 3);
	}

	#[pollster::test]
	async fn ordered() {
		let mut reactor = Reactor::<()>::new();

		struct MyEvent;
		impl Event for MyEvent {
			type Args<'a> = Vec<char>;
			type Return = ();
		}

		add_callback!(reactor, MyEvent => "A" => |_ctx, args| SmallBox::new(async move {
			args.push('A');
			ControlFlow::Continue(())
		}));
		add_callback!(reactor, MyEvent => "B" => |_ctx, args| SmallBox::new(async move {
			args.push('B');
			ControlFlow::Continue(())
		}), after "closureslop:D", after "closureslop:E");
		add_callback!(reactor, MyEvent => "C" => |_ctx, args| SmallBox::new(async move {
			args.push('C');
			ControlFlow::Continue(())
		}), before "closureslop:A");
		add_callback!(reactor, MyEvent => "D" => |_ctx, args| SmallBox::new(async move {
			args.push('D');
			ControlFlow::Continue(())
		}), after "closureslop:C", after "closureslop:A", before "closureslop:B");
		add_callback!(reactor, MyEvent => "E" => |_ctx, args| SmallBox::new(async move {
			args.push('E');
			ControlFlow::Continue(())
		}), after "closureslop:A");

		let mut x = Vec::new();
		reactor.trigger::<MyEvent>(&(), &mut x).await;
		assert_eq!(&x, &['C', 'A', 'D', 'E', 'B']);
	}
}
