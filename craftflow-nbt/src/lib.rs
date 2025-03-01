#![feature(portable_simd)]

//! NOTE THAT IT IS OF UPMOST IMPORTANCE THAT YOU ARE READING YOUR DATA INTO A

mod bytes_abstr;
mod nbt_bytes;
// mod byteswap;
// mod casts;
mod error;
// mod nbt_format;
// mod primitive;
// mod tag;
// mod value;

pub use error::{Error, Result};
// pub use value::{NbtString, NbtValue};

// /// helper function that advances a slice by n bytes
// fn advance<T>(s: &mut &mut [T], n: usize) {
// 	// this is very simple but i wanted to put it in a function because the code
// 	// is a clever workaround to the borrow checker and might not be easy to understand
// 	//
// 	// Basically it does the same as
// 	// *s = &mut s[1..];
// 	// but the above wouldnt compile, since it borrows the inner slice from the outer
// 	// and this results in the outer lifetime, which is not necessarily as long as the inner one,
// 	// so we cant assign like this.
// 	//
// 	// mem::take() doesnt borrow from the outer, instead just takes it, replacing it with an empty slice
// 	// and then we can have the inner lifetime, and assign it back
// 	*s = &mut std::mem::take(s)[n..];
// }
