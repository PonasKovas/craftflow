//! shared boilerplate code between tests and benchmarks

use craftflow_nbt::{
	Nbt, NbtByteArray, NbtIntArray, NbtList, NbtLongArray, NbtString, NbtValue, Tag,
};
use rand::distr::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::iter;

fn gen_random_string(rng: &mut StdRng, len_range: usize) -> NbtString {
	let length = rng.random_range(0..len_range);

	rng.sample_iter(&Alphanumeric)
		.take(length)
		.map(|c| c as char)
		.collect::<String>()
		.try_into()
		.unwrap()
}

#[allow(unused)]
pub fn gen_random_dyn_nbt(tags_n: usize) -> NbtValue {
	let mut rng = StdRng::seed_from_u64(0);
	let mut root_nbt = HashMap::new();

	let mut current_parent = &mut root_nbt;
	let mut i = 0;
	while i < tags_n {
		let k = gen_random_string(&mut rng, 64);
		match Tag::new(rng.random_range(1..12)).unwrap() {
			Tag::End => unreachable!(),
			Tag::Byte => {
				current_parent.insert(k, NbtValue::Byte(rng.random()));
			}
			Tag::Short => {
				current_parent.insert(k, NbtValue::Short(rng.random()));
			}
			Tag::Int => {
				current_parent.insert(k, NbtValue::Int(rng.random()));
			}
			Tag::Long => {
				current_parent.insert(k, NbtValue::Long(rng.random()));
			}
			Tag::Float => {
				current_parent.insert(k, NbtValue::Float(rng.random()));
			}
			Tag::Double => {
				current_parent.insert(k, NbtValue::Double(rng.random()));
			}
			Tag::String => {
				current_parent.insert(
					k,
					NbtValue::String(gen_random_string(&mut rng, 2usize.pow(16) - 1)),
				);
			}
			Tag::ByteArray => {
				let length = rng.random_range(0..1024 * 1024);
				current_parent.insert(
					k,
					NbtValue::ByteArray(NbtByteArray(
						(&mut rng).random_iter().take(length).collect(),
					)),
				);
			}
			Tag::IntArray => {
				let length = rng.random_range(0..512 * 1024);
				current_parent.insert(
					k,
					NbtValue::IntArray(NbtIntArray(
						(&mut rng).random_iter().take(length).collect(),
					)),
				);
			}
			Tag::LongArray => {
				let length = rng.random_range(0..1024);
				current_parent.insert(
					k,
					NbtValue::LongArray(NbtLongArray(
						(&mut rng).random_iter().take(length).collect(),
					)),
				);
			}
			Tag::List => {
				let length = rng.random_range(0..1024 * 1024);
				match Tag::new(rng.random_range(1..12)).unwrap() {
					Tag::Byte => {
						current_parent.insert(
							k,
							NbtValue::List(NbtList::Byte(
								(&mut rng).random_iter().take(length).collect(),
							)),
						);
					}
					Tag::Short => {
						current_parent.insert(
							k,
							NbtValue::List(NbtList::Short(
								(&mut rng).random_iter().take(length / 2).collect(),
							)),
						);
					}
					Tag::Int => {
						current_parent.insert(
							k,
							NbtValue::List(NbtList::Int(
								(&mut rng).random_iter().take(length / 4).collect(),
							)),
						);
					}
					Tag::Long => {
						current_parent.insert(
							k,
							NbtValue::List(NbtList::Long(
								(&mut rng).random_iter().take(length / 8).collect(),
							)),
						);
					}
					Tag::Float => {
						current_parent.insert(
							k,
							NbtValue::List(NbtList::Float(
								(&mut rng).random_iter().take(length / 4).collect(),
							)),
						);
					}
					Tag::Double => {
						current_parent.insert(
							k,
							NbtValue::List(NbtList::Double(
								(&mut rng).random_iter().take(length / 8).collect(),
							)),
						);
					}
					Tag::String => {
						current_parent.insert(
							k,
							NbtValue::List(NbtList::String(
								iter::from_fn(|| Some(gen_random_string(&mut rng, 512)))
									.take(length)
									.collect(),
							)),
						);
					}
					_ => continue, // just skip the other ones at least for now, too complex to generate them well
				}
			}
			Tag::Compound => {
				current_parent.insert(k.clone(), NbtValue::Compound(HashMap::new()));
				current_parent = current_parent.get_mut(&k).unwrap().expect_mut_compound();
			}
		}
		i += 1;
	}

	NbtValue::Compound(root_nbt)
}

#[allow(unused)]
pub fn roundtrip_test<T: Nbt + Debug + PartialEq>(value: &T) -> Result<(), Box<dyn Error>> {
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
