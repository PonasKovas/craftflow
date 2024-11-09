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
/// let my_key = "dynamic key".into(); // all strings (key or value) must be Cow<'a, str>
/// let compound = dyn_nbt!({
///    "key" : 5,
///    my_key: 123,
///    "annotated": #[byte] 1,
///    "complex_expr": 2i32.pow(8) + 49,
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
    // @list <vec ident> (<type annotation>) [remaining tokens]
    //
    // done list
    (@list $vec:ident () []) => {};
    // done list with trailing comma
    (@list $vec:ident () [,]) => {};
    // list value annotation
    (@list $vec:ident () [ #[$ty:tt] $($tt:tt)+ ]) => {
        $crate::dyn_nbt_internal!(@list $vec ($ty) [ $($tt)+ ]);
    };
    // list list
    (@list $vec:ident ($($ty:tt)?) [ [$($list:tt)*] $(, $($tt:tt)* )? ]) => {
        $vec.push($crate::dyn_nbt_internal!($(@annotated $ty)? [$($list)*]));
        $crate::dyn_nbt_internal!(@list $vec () [ $( $($tt)* )? ]);
    };
    // list compound (compounds cannot be annotated)
    (@list $vec:ident () [ {$($compound:tt)*} $(, $($tt:tt)* )? ]) => {
        $vec.push($crate::dyn_nbt_internal!( {$($compound)*} ));
        $crate::dyn_nbt_internal!(@list $vec () [ $( $($tt)* )? ]);
    };
    // list value
    (@list $vec:ident ($($ty:tt)?) [ $value:expr $(, $($tt:tt)* )? ]) => {
        $vec.push($crate::dyn_nbt_internal!($(@annotated $ty)? $value));
        $crate::dyn_nbt_internal!(@list $vec () [ $( $($tt)* )? ]);
    };

    // compound syntax
    // @compound <hashmap ident> (<key>) (<type annotation>) {remaining tokens}
    //
    // done compound
    (@compound $map:ident () () {}) => {};
    // done compound with trailing comma
    (@compound $map:ident () () {,}) => {};
    // compound literal key
    (@compound $map:ident () () { $key:literal : $($tt:tt)+ }) => {
        $crate::dyn_nbt_internal!(@compound $map (::std::borrow::Cow::Borrowed($key)) () { $($tt)+ });
    };
    // compound ident key
    (@compound $map:ident () () { $key:ident : $($tt:tt)+ }) => {
        $crate::dyn_nbt_internal!(@compound $map ($key) () { $($tt)+ });
    };
    // compound value annotation
    (@compound $map:ident ($key:expr) () { #[$ty:tt] $($tt:tt)+ }) => {
        $crate::dyn_nbt_internal!(@compound $map ($key) ($ty) { $($tt)+ });
    };
    // compound list
    (@compound $map:ident ($key:expr) ($($ty:tt)?) { [$($list:tt)*] $(, $($tt:tt)* )? }) => {
        $map.insert($key, $crate::dyn_nbt_internal!($(@annotated $ty)? [$($list)*]));
        $crate::dyn_nbt_internal!(@compound $map () () { $( $($tt)* )? });
    };
    // compound compound (compounds cannot be annotated)
    (@compound $map:ident ($key:expr) () { {$($compound:tt)*} $(, $($tt:tt)* )? }) => {
        $map.insert($key, $crate::dyn_nbt_internal!( {$($compound)*} ));
        $crate::dyn_nbt_internal!(@compound $map () () { $( $($tt)* )? });
    };
    // compound value
    (@compound $map:ident ($key:expr) ($($ty:tt)?) { $value:expr $(, $($tt:tt)* )? }) => {
        $map.insert($key, $crate::dyn_nbt_internal!($(@annotated $ty)? $value));
        $crate::dyn_nbt_internal!(@compound $map () () { $( $($tt)* )? });
    };

	// annotated type values
	(@annotated byte $val:expr) => {
		$crate::DynNBT::Byte($val)
	};
	(@annotated short $val:expr) => {
		$crate::DynNBT::Short($val)
	};
	(@annotated int $val:expr) => {
		$crate::DynNBT::Int($val)
	};
	(@annotated long $val:expr) => {
		$crate::DynNBT::Long($val)
	};
	(@annotated float $val:expr) => {
		$crate::DynNBT::Float($val)
	};
	(@annotated double $val:expr) => {
		$crate::DynNBT::Double($val)
	};
	(@annotated string $val:literal) => {
		$crate::DynNBT::String(::std::borrow::Cow::Borrowed($val))
	};
	(@annotated string $val:expr) => {
		$crate::DynNBT::String($val)
	};
	(@annotated byte_array [$($val:expr),* $(,)?]) => {
	    $crate::DynNBT::ByteArray(::std::borrow::Cow::Owned(vec![$($val),*]))
	};
	(@annotated int_array [$($val:expr),* $(,)?]) => {
        $crate::DynNBT::IntArray(::std::borrow::Cow::Owned(vec![$($val),*]))
	};
	(@annotated long_array [$($val:expr),* $(,)?]) => {
	    $crate::DynNBT::LongArray(::std::borrow::Cow::Owned(vec![$($val),*]))
	};

	// BASE CASES //
	// ////////// //

	// list
    ([ $( $tt:tt )* ]) => {{
    // ([$( $(#[$ty:tt])? $tt:tt ),* $(,)?]) => {{
        let list = $crate::DynNBT::List({
            #[allow(unused_mut)]
            let mut vec = ::std::vec::Vec::new();
            $crate::dyn_nbt_internal!(@list vec () [$($tt)*]);
            $crate::dynamic::DynNBTList::Owned(vec)
        });
        list.validate().expect("invalid nbt in dyn_nbt! macro");
        list
    }};
    // compound
    ({ $( $tt:tt )* }) => {{
        let map = $crate::DynNBT::Compound({
            #[allow(unused_mut)]
            let mut map = ::std::collections::HashMap::new();
            $crate::dyn_nbt_internal!(@compound map () () {$($tt)*});
            $crate::dynamic::DynNBTCompound::Owned(map)
        });
        map
    }};
	// annotated type values
	(#[$ty:tt] $($tt:tt)+) => {
	    $crate::dyn_nbt_internal!(@annotated $ty $($tt)+)
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
	use std::{borrow::Cow, collections::HashMap};

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
			DynNBT::List(vec![DynNBT::Int(1), DynNBT::Int(3), DynNBT::Int(5)].into())
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
			DynNBT::List(vec![DynNBT::Short(1), DynNBT::Short(3), DynNBT::Short(5)].into())
		);

		let spec_list = dyn_nbt!(
			#[long_array]
			[1, 3, 5]
		);
		assert_eq!(spec_list, DynNBT::LongArray(vec![1, 3, 5].into()));

		let simple_compound = dyn_nbt!({
			"first": 1,
			"second": 2,
		});
		assert_eq!(
			simple_compound,
			DynNBT::Compound({
				let mut map = HashMap::new();
				map.insert("first".into(), DynNBT::Int(1));
				map.insert("second".into(), DynNBT::Int(2));
				map.into()
			})
		);

		let key: Cow<'static, str> = "hii".into();
		let simple_compound2 = dyn_nbt!({
			"first": #[byte] 1,
			key: #[byte] 1+1,
		});
		assert_eq!(
			simple_compound2,
			DynNBT::Compound({
				let mut map = HashMap::new();
				map.insert("first".into(), DynNBT::Byte(1));
				map.insert("hii".into(), DynNBT::Byte(2));
				map.into()
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
					"first".into(),
					DynNBT::Compound(
						vec![(
							"inner0".into(),
							DynNBT::Compound(
								vec![(
									"inner1".into(),
									DynNBT::Compound(
										vec![(
											"inner2".into(),
											DynNBT::List(
												vec![DynNBT::Compound(
													vec![(
														"inner4".into(),
														DynNBT::List(vec![].into())
													)]
													.into_iter()
													.collect::<HashMap<_, _>>()
													.into()
												)]
												.into()
											)
										)]
										.into_iter()
										.collect::<HashMap<_, _>>()
										.into()
									)
								)]
								.into_iter()
								.collect::<HashMap<_, _>>()
								.into()
							)
						)]
						.into_iter()
						.collect::<HashMap<_, _>>()
						.into()
					)
				)]
				.into_iter()
				.collect::<HashMap<_, _>>()
				.into()
			)
		);
	}
}
