use closureslop::{Event, Reactor, callback, init, reg};
use std::ops::ControlFlow;

init!(ctx: ());
init!(group: "1", ctx: ());
init!(group: "2", ctx: ());
init!(group: "ordered", ctx: ());

struct Adder;
impl Event for Adder {
	type Args<'a> = String;
	type Return = ();
}

#[pollster::test]
async fn base() {
	let mut reactor = Reactor::new();

	#[callback(event: Adder)]
	async fn a(_ctx: &(), acc: &mut String) -> ControlFlow<()> {
		*acc += "a";
		ControlFlow::Continue(())
	}

	reg!(to: &mut reactor);

	let mut acc = String::new();
	reactor.trigger::<Adder>(&(), &mut acc).await;

	assert_eq!(acc, "a")
}

#[pollster::test]
async fn with_groups() {
	let mut reactor = Reactor::new();

	#[callback(event: Adder, group: "1")]
	async fn b(_ctx: &(), acc: &mut String) -> ControlFlow<()> {
		*acc += "b";
		ControlFlow::Continue(())
	}

	#[callback(event: Adder, group: "2")]
	async fn c(_ctx: &(), acc: &mut String) -> ControlFlow<()> {
		*acc += "c";
		ControlFlow::Continue(())
	}

	reg!(to: &mut reactor, group: "1");
	reg!(to: &mut reactor, group: "2");

	let mut acc = String::new();
	reactor.trigger::<Adder>(&(), &mut acc).await;

	assert_eq!(acc, "bc")
}

#[pollster::test]
async fn with_order() {
	let mut reactor = Reactor::new();

	#[callback(event: Adder, group: "ordered", after: "proc_macro_test:second")]
	async fn third(_ctx: &(), acc: &mut String) -> ControlFlow<()> {
		*acc += "3";
		ControlFlow::Continue(())
	}
	#[callback(event: Adder, group: "ordered")]
	async fn first(_ctx: &(), acc: &mut String) -> ControlFlow<()> {
		*acc += "1";
		ControlFlow::Continue(())
	}
	#[callback(event: Adder, group: "ordered", after: "proc_macro_test:first", before: "proc_macro_test:third")]
	async fn second(_ctx: &(), acc: &mut String) -> ControlFlow<()> {
		*acc += "2";
		ControlFlow::Continue(())
	}

	reg!(to: &mut reactor, group: "ordered");

	let mut acc = String::new();
	reactor.trigger::<Adder>(&(), &mut acc).await;

	assert_eq!(acc, "123")
}
