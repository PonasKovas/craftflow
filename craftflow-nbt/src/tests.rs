use crate::{
	Nbt,
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

	test(line!(), &"Hello, world!".to_string());
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
				.map(|s| s.to_owned())
				.collect(),
		),
	);

	// #[derive(Serialize, Deserialize, Debug, PartialEq)]
	// struct SimpleStruct {
	// 	#[serde(default)]
	// 	first: Option<usize>,
	// 	second: f64,
	// }
	// test(
	// 	line!(),
	// 	&SimpleStruct {
	// 		first: None,
	// 		second: 9125123.213,
	// 	},
	// );
	// test(
	// 	line!(),
	// 	&SimpleStruct {
	// 		first: Some(123456789),
	// 		second: 9125123.213,
	// 	},
	// );

	// #[derive(Serialize, Deserialize, Debug, PartialEq)]
	// struct ComplexStruct {
	// 	#[serde(default)]
	// 	first: Option<usize>,
	// 	#[serde(default)]
	// 	second: Option<usize>,
	// 	third: Either,
	// 	inner: InnerStruct,
	// }
	// #[derive(Serialize, Deserialize, Debug, PartialEq)]
	// struct InnerStruct {
	// 	first: String,
	// 	second: Vec<u32>,
	// 	third: HashMap<String, InnerStruct>,
	// }

	// test(
	// 	line!(),
	// 	&ComplexStruct {
	// 		first: None,
	// 		second: Some(567),
	// 		third: Either::Left(57),
	// 		inner: InnerStruct {
	// 			first: format!("banana🍌"),
	// 			second: vec![0xB00B135, 0xFACE, 0xFEED],
	// 			third: {
	// 				let mut map = HashMap::new();
	// 				map.insert(
	// 					format!(
	// 						"why did the scarecrow win an award? because he was outstanding in his field!"
	// 					),
	// 					InnerStruct {
	// 						first: format!("testing... testing... 1, 2, 3... is this thing on?"),
	// 						second: vec![],
	// 						third: HashMap::new(),
	// 					},
	// 				);
	// 				map.insert(
	// 					format!("i like big bytes and i cannot lie!"),
	// 					InnerStruct {
	// 						first: format!("this is not a test"),
	// 						second: vec![5],
	// 						third: HashMap::new(),
	// 					},
	// 				);
	// 				map
	// 			},
	// 		},
	// 	},
	// );
}

#[test]
fn bigtest() {
	let bytes = include_bytes!("../bigtest.nbt");
	let mut slice = &bytes[..];
	let (name, _val) = NbtValue::nbt_read_named(&mut slice).unwrap();
	assert!(slice.is_empty());
	assert_eq!(name, "Level");
	// panic!("{:#?}", _val);
}
