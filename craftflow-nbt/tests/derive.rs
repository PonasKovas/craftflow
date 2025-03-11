use craftflow_nbt::Nbt;
use std::{collections::HashMap, error::Error, fmt::Debug};

fn roundtrip<T: Nbt + Debug + PartialEq>(value: &T) -> Result<(), Box<dyn Error>> {
	let mut buffer = Vec::new();
	let l = value.nbt_write(&mut buffer);

	if l != buffer.len() {
		Err(format!("written {l} != {} buffer len ", buffer.len()))?
	}

	let mut slice = &buffer[..];
	let reconstructed: T = match T::nbt_read(&mut slice) {
		Ok(r) => {
			if !slice.is_empty() {
				Err(format!(
					"buffer not empty:\n{}",
					hexdump::hexdump_iter(slice)
						.fold(String::new(), |acc, line| { acc + &*line + "\n" })
				))?
			}

			r
		}
		Err(e) => Err(format!(
			"Failed to deserialize {value:?}: {:?}:\n{}",
			e,
			hexdump::hexdump_iter(&buffer).fold(String::new(), |acc, line| { acc + &*line + "\n" })
		))?,
	};

	if value != &reconstructed {
		Err(format!(
			"reconstructed doesnt match\nOriginal: {value:?}\n\nReconstructed: {reconstructed:?}"
		))?
	}

	Ok(())
}

#[test]
fn derive_generics() {
	#[derive(Nbt, Debug, PartialEq)]
	struct Generics<T1, T2> {
		a: T1,
		b: T2,
		c: i32,
	}

	if let Err(e) = roundtrip(&Generics {
		a: format!("first!"),
		b: vec![1i8, 2, 3],
		c: 123456789,
	}) {
		panic!("{e}");
	}
}

#[test]
fn nested_lists() {
	#[derive(Nbt, Debug, PartialEq)]
	struct NestedLists {
		a: Vec<Vec<Vec<f64>>>,
	}

	if let Err(e) = roundtrip(&NestedLists {
		a: vec![vec![vec![1., 2., 3., 4.], vec![5., 6., 7., 8.]], vec![]],
	}) {
		panic!("{e}");
	}
}

#[test]
fn nested_structures() {
	#[derive(Nbt, Debug, PartialEq)]
	struct Outer {
		a: Inner,
		b: HashMap<String, f32>,
	}
	#[derive(Nbt, Debug, PartialEq)]
	struct Inner {
		a: Innermost,
		b: HashMap<String, HashMap<String, Innermost>>,
	}
	#[derive(Nbt, Debug, PartialEq)]
	struct Innermost {
		a: i8,
		b: i8,
	}

	let v = Outer {
		a: Inner {
			a: Innermost { a: -128, b: 127 },
			b: {
				let mut m = HashMap::new();
				m.insert(format!("YES"), HashMap::new());
				m.insert(format!("OMG YES"), {
					let mut m = HashMap::new();
					m.insert(format!("SO DEEP!!!"), Innermost { a: 1, b: -2 });
					m
				});
				m
			},
		},
		b: {
			let mut m = HashMap::new();
			m.insert(format!("I HECKING LOVE NBT"), 9999999999999.);
			m
		},
	};

	if let Err(e) = roundtrip(&v) {
		panic!("{e}");
	}
}
