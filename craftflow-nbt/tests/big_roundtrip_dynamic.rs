#[path = "../shared.rs"]
mod shared;

#[test]
fn big_roundtrip_dynamic() {
	let nbt = shared::gen_random_dyn_nbt(200);

	if let Err(e) = shared::roundtrip_test(&nbt) {
		panic!("{e}");
	}
}
