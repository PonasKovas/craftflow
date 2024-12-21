# Notes for the future

This file should list things that may need attention or to be revisited in the future, such as workarounds and hacks
to compiler or library limitations that may be possible to fix/simplify in the future.

### 2024-12 Lifetimes in GATs + async (compiler limitation)

At the time of writing: `rustc 1.85.0-nightly`.

Lifetimes in GATs (`craftflow::reactor::Event` trait, `Args<'a>` associated type) and async functions (reactor callbacks
being async, and therefore `Reactor::event` method being async) do not play well together.

It fails to compile with cryptic errors like this:

```rust
error[E0308]: mismatched types
  --> craftflow/src/packet_events.rs:56:2
   |
56 | /     assert_send(
57 | |         craftflow
58 | |             .reactor
59 | |             .event::<<P as PacketToEventPointer>::Event>(craftflow, &mut args),
60 | |     );
   | |_____^ one type is more general than the other
   |
   = note: expected associated type `<<P as PacketToEventPointer<'_>>::Event as reactor::Event>::Args<'_>`
              found associated type `<<P as PacketToEventPointer<'_>>::Event as reactor::Event>::Args<'_>`
note: the lifetime requirement is introduced here
  --> craftflow/src/packet_events.rs:55:20
   |
55 |     fn assert_send<T: Send>(_: T) {}
   |                       ^^^^

error: lifetime may not live long enough
  --> craftflow/src/packet_events.rs:48:1
   |
45 |   async fn helper<'a, 'b, P>(craftflow: &'a CraftFlow, conn_id: u64, packet: P) -> (bool, P)
   |                       -- lifetime `'b` defined here
...
48 | / {
49 | |     trace!(
50 | |         "{} event",
51 | |         std::any::type_name::<<P as PacketToEventPointer>::Event>()
...  |
71 | | }
   | |_^ returning this value requires that `'b` must outlive `'static`
```

I suspect that the issue is caused by limitations of the compiler in relation to GATs and async functions.
https://github.com/rust-lang/rust/issues/110338
There appear to be many related issues with no general solutions.

It is expected that these issues be solved with the [next trait solver](https://github.com/rust-lang/rust/issues/107374).
As of now, trying to compile this with `-Znext-solver` results in an ICE.

For now, I will attempt to work around this by removing the lifetime from the GAT, and instead emulating it
with a pattern like this:

```rust
trait Event: Any + for<'a> EventArgs<'a> {
	/// The type of the return value of the event
	type Return;
}

trait EventArgs<'a> {
	/// The type of the arguments that the event will receive
	type Args;
}
```

It might be a good idea to revisit this and check if it could be simplified back, once next solver is more usable.

Last commit that attempted to use GATs: `ed0adf675755c12b9f7377e668b676ec9c8afa7a`
