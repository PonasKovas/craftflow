use craftflow_nbt::Nbt;
use std::{collections::HashMap, fmt::Debug};

#[path = "../shared.rs"]
mod shared;

#[test]
fn derive_generics() {
	#[derive(Nbt, Debug, PartialEq)]
	struct Generics<T1, T2> {
		a: T1,
		b: T2,
		c: i32,
	}

	if let Err(e) = shared::roundtrip_test(&Generics {
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

	if let Err(e) = shared::roundtrip_test(&NestedLists {
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

	if let Err(e) = shared::roundtrip_test(&v) {
		panic!("{e}");
	}
}

#[test]
fn derive_optional_fields() {
	#[derive(Nbt, Debug, PartialEq)]
	struct Outer {
		a: i64,
		b: Option<i8>,
		c: Inner,
	}

	#[derive(Nbt, Debug, PartialEq)]
	struct Inner {
		a: Option<String>,
		b: Option<f64>,
	}

	if let Err(e) = shared::roundtrip_test(&Outer {
		a: 123456789,
		b: None,
		c: Inner {
			a: None,
			b: Some(4984231489165.4),
		},
	}) {
		panic!("{e}");
	}
	if let Err(e) = shared::roundtrip_test(&Outer {
		a: 123456789,
		b: Some(123),
		c: Inner {
			a: Some(format!("HELLOOOOOO")),
			b: None,
		},
	}) {
		panic!("{e}");
	}
}
