/// Convenience macro for creating [`NbtValue`][crate::NbtValue].
#[macro_export]
macro_rules! nbt {
	($($nbt:tt)+) => {
	    $crate::_nbt_internal!($($nbt)+)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _nbt_internal {
    // done list
    (@list $vec:ident []) => {};
    // done list with trailing comma
    (@list $vec:ident [,]) => {};

    // list list
    (@list $vec:ident [ [$($list:tt)*] $(, $($tt:tt)* )? ]) => {
        $vec.push($crate::_nbt_internal!([$($list)*]));
        $crate::_nbt_internal!(@list $vec [ $( $($tt)* )? ]);
    };
    // list compound
    (@list $vec:ident [ {$($compound:tt)*} $(, $($tt:tt)* )? ]) => {
        $vec.push($crate::_nbt_internal!( {$($compound)*} ));
        $crate::_nbt_internal!(@list $vec [ $( $($tt)* )? ]);
    };
    // list value
    (@list $vec:ident [ $value:expr $(, $($tt:tt)* )? ]) => {
        $vec.push($value);
        $crate::_nbt_internal!(@list $vec [ $( $($tt)* )? ]);
    };

    // done compound
    (@compound $map:ident () {}) => {};
    // done compound with trailing comma
    (@compound $map:ident () {,}) => {};

    // compound literal key
    (@compound $map:ident () { $key:literal : $($tt:tt)+ }) => {
        $crate::_nbt_internal!(@compound $map ($crate::NbtString::from_str($key).unwrap()) { $($tt)+ });
    };
    // compound ident key
    (@compound $map:ident () { $key:ident : $($tt:tt)+ }) => {
        $crate::_nbt_internal!(@compound $map ($key) { $($tt)+ });
    };
    // compound list
    (@compound $map:ident ($key:expr) { [$($list:tt)*] $(, $($tt:tt)* )? }) => {
        $map.insert($key, $crate::_nbt_internal!([$($list)*]));
        $crate::_nbt_internal!(@compound $map () { $( $($tt)* )? });
    };
    // compound compound
    (@compound $map:ident ($key:expr) { {$($compound:tt)*} $(, $($tt:tt)* )? }) => {
        $map.insert($key, $crate::_nbt_internal!( {$($compound)*} ));
        $crate::_nbt_internal!(@compound $map () { $( $($tt)* )? });
    };
    // compound value
    (@compound $map:ident ($key:expr) { $value:expr $(, $($tt:tt)* )? }) => {
        $map.insert($key, $crate::_nbt_internal!( $value ));
        $crate::_nbt_internal!(@compound $map () { $( $($tt)* )? });
    };

	// BASE CASES //
	// ////////// //

	// list
    ([ $( $tt:tt )* ]) => {{
        let list = $crate::NbtValue::List({
            #[allow(unused_mut)]
            let mut vec = ::std::vec::Vec::new();
            $crate::_nbt_internal!(@list vec [$($tt)*]);
            vec.into()
        });
        list
    }};
    // compound
    ({ $( $tt:tt )* }) => {{
        let map = $crate::NbtValue::Compound({
            #[allow(unused_mut)]
            let mut map = ::std::collections::HashMap::new();
            $crate::_nbt_internal!(@compound map () {$($tt)*});
            map
        });
        map
    }};
	// Anything that NbtValue implements From<T> for
	($val:expr) => {
		$crate::NbtValue::from($val)
	};
}
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_macro() {
        let _ = nbt!(8);
        let _ = nbt!(3.0);
        let _ = nbt!([1,2,3]);
        let _ = nbt!({ "test" : 8 });
        let _ = nbt!({ "test" : 8, "wow": NbtByteArray(vec![1, 2, 3]) });
        let _ = nbt!({ "outer": { "inner": { "innermost": 7.0 }}});
    }
}