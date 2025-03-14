use craftflow_nbt::{NbtByteArray, NbtIntArray, NbtList, NbtLongArray, NbtRead, NbtValue, nbtstr};
use shared::roundtrip_test;

#[path = "../shared.rs"]
mod shared;

#[test]
fn test_roundtrip() {
	roundtrip_test(&nbtstr!("Hello, world!").to_owned()).unwrap();
	roundtrip_test(&3.1456789f32).unwrap(); // this is the actual PI value
	roundtrip_test(&f64::INFINITY).unwrap();
	roundtrip_test(&i8::MIN).unwrap();
	roundtrip_test(&i16::MIN).unwrap();
	roundtrip_test(&i32::MIN).unwrap();
	roundtrip_test(&i64::MIN).unwrap();
	roundtrip_test(&NbtIntArray(vec![1, 2, 3, 4, 5])).unwrap();
	roundtrip_test(&NbtLongArray(vec![1, 2, 3, 4, 5])).unwrap();
	roundtrip_test(&NbtByteArray(vec![1, 2, 3, 4, 5])).unwrap();
	roundtrip_test(&NbtList::Short(vec![1, 2, 3, 4, 5])).unwrap();
	roundtrip_test(&NbtList::String(
		["hello", "from", "earth"]
			.into_iter()
			.map(|s| s.try_into().unwrap())
			.collect(),
	))
	.unwrap();
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
