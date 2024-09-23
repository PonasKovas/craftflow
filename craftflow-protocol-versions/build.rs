//! This build script will generate the packets for all versions of the protocol

#[path = "build/mod.rs"]
mod build;

const GIT_URL: &str = "https://github.com/PrismarineJS/minecraft-data.git";
const GIT_COMMIT: &str = "3f429fb293184d1b7a1887e7addad5dc8c77ead1";
const VERSIONS: &[u32] = &[
	767, 766, 765, 764, 763, 762, 761, 760, 759, 758, 757, 756, 755, 754, 753, 751, 736, 735, 578,
	575, 573, 498, 490, 485, 480, 477, 404, 401, 393, 340, 338, 335, 316, 315, 210, 110, 109, 108,
	107, 47, 5,
];

pub fn main() {
	println!("cargo::rerun-if-changed=build.rs");

	build::main()
}
