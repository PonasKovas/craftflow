#[macro_export]
macro_rules! add_callback {
	($reactor:expr, $event:ty => $name:expr => $callback:expr $(, $($order:tt)* )?) => {
		#[allow(unused_mut)]
		let mut after = Vec::new();
		#[allow(unused_mut)]
		let mut before = Vec::new();

		$(
			$crate::add_callback!(@order: after, before => [ $($order)* ]);
		)?

		$reactor.add_callback::<$event, _>(
			format!("{}:{}", env!("CARGO_PKG_NAME"), $name),
			after,
			before,
			$callback,
		);
	};
	(@order: $after:ident, $before:ident => []) => {};
	(@order: $after:ident, $before:ident => [after $target:expr $(, $($order:tt)* )?]) => {
		$after.push($target.to_string());
		$(
			$crate::add_callback!(@order: $after, $before => [ $($order)* ]);
		)?
	};
	(@order: $after:ident, $before:ident => [before $target:expr $(, $($order:tt)* )?]) => {
		$before.push($target.to_string());
		$(
			$crate::add_callback!(@order: $after, $before => [ $($order)* ]);
		)?
	};
}

// re-exports used in closureslop-macros proc macros expansions

pub use linkme;
pub use smallbox;
