use crate::{
	Nbt, NbtString,
	nbtvalue::{NbtByteArray, NbtIntArray, NbtList, NbtLongArray, NbtValue},
};
use core::f64;
use std::fmt::Debug;

#[test]
fn test_roundtrip() {
	fn test<T: Nbt + Debug + PartialEq>(line: u32, value: &T) {
		let mut buffer = Vec::new();
		let l = value.nbt_write(&mut buffer);
		assert_eq!(l, buffer.len());

		let mut slice = &buffer[..];
		let reconstructed: T = match T::nbt_read(&mut slice) {
			Ok(r) => {
				if !slice.is_empty() {
					hexdump::hexdump(slice);
					panic!("buffer not empty: line {line}");
				}

				r
			}
			Err(e) => {
				hexdump::hexdump(&buffer);
				panic!("Failed to deserialize {value:?}: {:?} (line {line})", e,)
			}
		};

		assert_eq!(value, &reconstructed, "line {line}");
	}

	test(line!(), &NbtString::from_str("Hello, world!").unwrap());
	test(line!(), &3.1456789f32); // this is the actual PI value
	test(line!(), &f64::INFINITY);
	test(line!(), &i8::MIN);
	test(line!(), &i16::MIN);
	test(line!(), &i32::MIN);
	test(line!(), &i64::MIN);
	test(line!(), &NbtIntArray(vec![1, 2, 3, 4, 5]));
	test(line!(), &NbtLongArray(vec![1, 2, 3, 4, 5]));
	test(line!(), &NbtByteArray(vec![1, 2, 3, 4, 5]));
	test(line!(), &NbtList::Short(vec![1, 2, 3, 4, 5]));
	test(
		line!(),
		&NbtList::String(
			["hello", "from", "earth"]
				.into_iter()
				.map(|s| s.try_into().unwrap())
				.collect(),
		),
	);
}

#[test]
fn predefined() {
	let bytes = include_bytes!("../bigtest.nbt");
	let mut slice = &bytes[..];
	let (name, _val) = NbtValue::nbt_read_named(&mut slice).unwrap();
	assert!(slice.is_empty());
	assert_eq!(name, "Level");

	let bytes = include_bytes!("../complex_player.nbt");
	let mut slice = &bytes[..];
	let (name, _val) = NbtValue::nbt_read_named(&mut slice).unwrap();
	assert!(slice.is_empty());
	assert_eq!(name, "");

	// panic!("{name:?}\n{:#?}", _val);
}
