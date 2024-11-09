/// Construct [`DynNBT`][crate::DynNBT] in place conveniently.
///
/// All lists must have elements of the same type - if they're not,
/// the macro will panic (at runtime).
///
/// ```
/// # use craftflow_nbt::dyn_nbt;
/// let simple = dyn_nbt!(5); // DynNBT::Int(5)
/// let annotated = dyn_nbt!(#[short] 2); // DynNBT::Short(2)
/// let list = dyn_nbt!([1, 2, 3]); // DynNBT::List(vec![DynNBT::Int(1), DynNBT::Int(2), DynNBT::Int(3)])
/// let spec_list = dyn_nbt!(#[long_array] [1, 2, 3]); // DynNBT::LongArray(vec![1, 2, 3])
///
/// let my_key = "dynamic key".to_string();
/// let compound = dyn_nbt!({
///    "key" : 5,
///    my_key: 123,
///    "annotated": #[byte] 1,
///    // if you want the expression to be something more complex, use parentheses
///    "complex_expr": ("method calls".to_uppercase() + "and more"),
///    "list_with_annotation": [#[byte] 1, #[byte] 2, #[byte] 3], // all must be the same type
///    "inner": {
///        "key": true, // NBT doesn't really have bool but you can you use it here and
///    }                // it will be converted to a byte.
/// });
/// ```
#[macro_export]
macro_rules! dyn_nbt {
	($($nbt:tt)+) => {
	    $crate::dyn_nbt_internal!($($nbt)+)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! dyn_nbt_internal {
    // list syntax
    // @list <vec ident> [remaining tokens]
    //
    // done list
    (@list $vec:ident []) => {};
    // done list with trailing comma
    (@list $vec:ident [,]) => {};
    // list value with annotation
    (@list $vec:ident [ #[$ty:tt] $value:tt $(, $($tt:tt)* )? ]) => {
        $vec.push($crate::dyn_nbt_internal!(@annotated $ty $value));
        $crate::dyn_nbt_internal!(@list $vec [ $( $($tt)* )? ]);
    };
    // list value without annotation
    (@list $vec:ident [ $value:tt $(, $($tt:tt)* )? ]) => {
        $vec.push($crate::dyn_nbt_internal!($value));
        $crate::dyn_nbt_internal!(@list $vec [ $( $($tt)* )? ]);
    };
    // compound syntax
    // @compound <hashmap ident> (<key>) {remaining tokens}
    //
    // done compound
    (@compound $map:ident () {}) => {};
    // done compound with trailing comma
    (@compound $map:ident () {,}) => {};
    // compound literal key
    (@compound $map:ident () { $key:literal : $($tt:tt)+ }) => {
        $crate::dyn_nbt_internal!(@compound $map ({$key.to_string()}) { $($tt)+ });
    };
    // compound with ident key
    (@compound $map:ident () { $key:ident : $($tt:tt)+ }) => {
        $crate::dyn_nbt_internal!(@compound $map ($key) { $($tt)+ });
    };
    // compound value with annotation
    (@compound $map:ident ($key:tt) { #[$ty:tt] $value:tt $(, $($tt:tt)* )? }) => {
        $map.insert($key, $crate::dyn_nbt_internal!(@annotated $ty $value));
        $crate::dyn_nbt_internal!(@compound $map () { $( $($tt)* )? });
    };
    // compound value without annotation
    (@compound $map:ident ($key:tt) { $value:tt $(, $($tt:tt)* )? }) => {
        $map.insert($key, $crate::dyn_nbt_internal!($value));
        $crate::dyn_nbt_internal!(@compound $map () { $( $($tt)* )? });
    };
	// annotated type values
	(@annotated byte $val:tt) => {
		$crate::DynNBT::Byte($val)
	};
	(@annotated short $val:tt) => {
		$crate::DynNBT::Short($val)
	};
	(@annotated int $val:tt) => {
		$crate::DynNBT::Int($val)
	};
	(@annotated long $val:tt) => {
		$crate::DynNBT::Long($val)
	};
	(@annotated float $val:tt) => {
		$crate::DynNBT::Float($val)
	};
	(@annotated double $val:tt) => {
		$crate::DynNBT::Double($val)
	};
	(@annotated string $val:tt) => {
		$crate::DynNBT::String($val)
	};
	(@annotated byte_array [$($val:tt),* $(,)?]) => {
	    $crate::DynNBT::ByteArray(vec![$($val),*])
	};
	(@annotated int_array [$($val:tt),* $(,)?]) => {
        $crate::DynNBT::IntArray(vec![$($val),*])
	};
	(@annotated long_array [$($val:tt),* $(,)?]) => {
	    $crate::DynNBT::LongArray(vec![$($val),*])
	};
	// list
    ([ $( $tt:tt )* ]) => {{
    // ([$( $(#[$ty:tt])? $tt:tt ),* $(,)?]) => {{
        let list = $crate::DynNBT::List({
            #[allow(unused_mut)]
            let mut vec = ::std::vec::Vec::new();
            $crate::dyn_nbt_internal!(@list vec [$($tt)*]);
            vec
        });
        list.validate().expect("invalid nbt in dyn_nbt! macro");
        list
    }};
    // compound
    ({ $( $tt:tt )* }) => {{
        let map = $crate::DynNBT::Compound({
            #[allow(unused_mut)]
            let mut map = ::std::collections::HashMap::new();
            $crate::dyn_nbt_internal!(@compound map () {$($tt)*});
            map
        });
        map
    }};
	// annotated type values
	(#[$ty:tt] $val:tt) => {
	    $crate::dyn_nbt_internal!(@annotated $ty $val)
	};
	// Not annotated type values
	// Anything that DynNBT implements From<T> for
	($val:expr) => {
		$crate::DynNBT::from($val)
	};
}

#[cfg(test)]
mod tests {
	use crate::DynNBT;
	use std::collections::HashMap;

	#[test]
	fn test_macro() {
		let simple = dyn_nbt!(1);
		assert_eq!(simple, DynNBT::Int(1));

		let annotated = dyn_nbt!(
			#[byte]
			1
		);
		assert_eq!(annotated, DynNBT::Byte(1));

		let list = dyn_nbt!([1, 3, 5]);
		assert_eq!(
			list,
			DynNBT::List(vec![DynNBT::Int(1), DynNBT::Int(3), DynNBT::Int(5)])
		);

		let annotated_list = dyn_nbt!([
			#[short]
			1,
			#[short]
			3,
			#[short]
			5
		]);
		assert_eq!(
			annotated_list,
			DynNBT::List(vec![DynNBT::Short(1), DynNBT::Short(3), DynNBT::Short(5)])
		);

		let spec_list = dyn_nbt!(
			#[long_array]
			[1, 3, 5]
		);
		assert_eq!(spec_list, DynNBT::LongArray(vec![1, 3, 5]));

		let simple_compound = dyn_nbt!({
			"first": 1,
			"second": 2,
		});
		assert_eq!(
			simple_compound,
			DynNBT::Compound({
				let mut map = HashMap::new();
				map.insert("first".to_string(), DynNBT::Int(1));
				map.insert("second".to_string(), DynNBT::Int(2));
				map
			})
		);

		let key = format!("hii");
		let simple_compound2 = dyn_nbt!({
			"first": #[byte] 1,
			key: #[byte] (1+1),
		});
		assert_eq!(
			simple_compound2,
			DynNBT::Compound({
				let mut map = HashMap::new();
				map.insert("first".to_string(), DynNBT::Byte(1));
				map.insert("hii".to_string(), DynNBT::Byte(2));
				map
			})
		);

		let complex_compound = dyn_nbt!({
			"first": {
				"inner0": {
					"inner1": {
						"inner2": [
							{
								"inner4": []
							}
						]
					}
				}
			}
		});
		assert_eq!(
			complex_compound,
			DynNBT::Compound(
				vec![(
					"first".to_string(),
					DynNBT::Compound(
						vec![(
							"inner0".to_string(),
							DynNBT::Compound(
								vec![(
									"inner1".to_string(),
									DynNBT::Compound(
										vec![(
											"inner2".to_string(),
											DynNBT::List(vec![DynNBT::Compound(
												vec![("inner4".to_string(), DynNBT::List(vec![]))]
													.into_iter()
													.collect()
											)])
										)]
										.into_iter()
										.collect()
									)
								)]
								.into_iter()
								.collect()
							)
						)]
						.into_iter()
						.collect()
					)
				)]
				.into_iter()
				.collect()
			)
		);
	}
}
