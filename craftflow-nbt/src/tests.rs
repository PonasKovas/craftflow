use crate::{
	arrays::{ByteArray, IntArray, LongArray},
	from_slice, to_writer,
};
use core::f64;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

#[test]
fn test_roundtrip() {
	fn test<T: Serialize + for<'a> Deserialize<'a> + Debug + PartialEq>(line: u32, value: &T) {
		let mut buffer = Vec::new();
		if let Err(e) = to_writer(&mut buffer, value) {
			panic!("Failed to serialize {value:?}: {:?} (line {line})", e);
		}

		let reconstructed: T = match from_slice(&buffer) {
			Ok(r) => r,
			Err(e) => panic!(
				"Failed to deserialize {value:?} from {buffer:x?}: {:?} (line {line})",
				e
			),
		};

		assert_eq!(value, &reconstructed, "line {line}");
	}

	test(line!(), &true);
	test(line!(), &false);
	test(line!(), &Some(123u32));
	test(line!(), &None::<String>);
	test(line!(), &"Hello, world!".to_string());
	test(line!(), &3.1456789f32); // this is the actual PI value
	test(line!(), &f64::INFINITY);
	test(line!(), &u8::MAX);
	test(line!(), &u16::MAX);
	test(line!(), &u32::MAX);
	test(line!(), &u64::MAX);
	test(line!(), &i8::MIN);
	test(line!(), &i16::MIN);
	test(line!(), &i32::MIN);
	test(line!(), &i64::MIN);
	test(line!(), &[1, 2, 3, 4, 5]);
	test(line!(), &vec![1, 2, 3, 4, 5]);
	test(line!(), &[0xDEu8, 0xAD, 0xBE, 0xEF]);
	test(
		line!(),
		&[format!("hello"), format!("from"), format!("earth!")],
	);
	test(line!(), &ByteArray([9u8, 8, 7, 6, 5, 4, 3, 2, 1, 0]));
	test(line!(), &IntArray([9u32, 8, 7, 6, 5, 4, 3, 2, 1, 0]));
	test(line!(), &LongArray([9u64, 8, 7, 6, 5, 4, 3, 2, 1, 0]));

	#[derive(Serialize, Deserialize, Debug, PartialEq)]
	#[serde(untagged)]
	enum Either {
		Left(u32),
		Right(String),
		Struct { name: String, value: u32 },
	}
	test(line!(), &Either::Left(123));
	test(line!(), &Either::Right(format!("i love minecraft :D")));
	test(
		line!(),
		&Either::Struct {
			name: format!("we are minecraft"),
			value: 12512738,
		},
	);

	#[derive(Serialize, Deserialize, Debug, PartialEq)]
	struct ComplexStruct {
		#[serde(default)]
		second: Option<usize>,
		third: Either,
		inner: InnerStruct,
	}
	#[derive(Serialize, Deserialize, Debug, PartialEq)]
	struct InnerStruct {
		first: String,
		second: Vec<u32>,
		third: HashMap<String, InnerStruct>,
	}

	test(
		line!(),
		&ComplexStruct {
			second: Some(567),
			third: Either::Left(57),
			inner: InnerStruct {
				first: format!("banana🍌"),
				second: vec![0xB00B135, 0xFACE, 0xFEED],
				third: {
					let mut map = HashMap::new();
					map.insert(
    					format!("why did the scarecrow win an award? because he was outstanding in his field!"),
    					InnerStruct { first: format!("testing... testing... 1, 2, 3... is this thing on?"), second: vec![], third: HashMap::new() }
					);
					map.insert(
						format!("i like big bytes and i cannot lie!"),
						InnerStruct {
							first: format!("this is not a test"),
							second: vec![5],
							third: HashMap::new(),
						},
					);
					map
				},
			},
		},
	);
}
