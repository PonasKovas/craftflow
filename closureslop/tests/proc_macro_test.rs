use closureslop::{Event, Reactor, callback, init, reg};
use std::ops::ControlFlow;

init!(ctx: ());
init!(group: "1", ctx: ());
init!(group: "2", ctx: ());
init!(group: "ordered", ctx: ());
init!(group: "m1", ctx: ());
init!(group: "m2", ctx: ());

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

	reg!(to: reactor);

	let mut acc = String::new();
	let _ = reactor.trigger::<Adder>(&(), &mut acc).await;

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

	reg!(to: reactor, group: "1");
	reg!(to: reactor, group: "2");

	let mut acc = String::new();
	let _ = reactor.trigger::<Adder>(&(), &mut acc).await;

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

	reg!(to: reactor, group: "ordered");

	let mut acc = String::new();
	let _ = reactor.trigger::<Adder>(&(), &mut acc).await;

	assert_eq!(acc, "123")
}

#[pollster::test]
async fn multiple_groups() {
	let mut reactor1 = Reactor::new();
	let mut reactor2 = Reactor::new();

	#[callback(event: Adder, group: "m1")]
	#[callback(event: Adder, group: "m2")]
	async fn foo(_ctx: &(), acc: &mut String) -> ControlFlow<()> {
		*acc += "ye";
		ControlFlow::Continue(())
	}

	reg!(to: reactor1, group: "m1");
	reg!(to: reactor2, group: "m2");

	let mut acc = String::new();
	let _ = reactor1.trigger::<Adder>(&(), &mut acc).await;
	let _ = reactor2.trigger::<Adder>(&(), &mut acc).await;

	assert_eq!(acc, "yeye")
}
