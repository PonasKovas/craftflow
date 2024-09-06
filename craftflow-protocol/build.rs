//! This build script will generate the packets for the protocol

#[path = "build/mod.rs"]
mod build;

fn main() {
	println!("cargo::rerun-if-changed=packets/");
	println!("cargo::rerun-if-changed=protocol.toml");

	// First handle the main protocol info file which includes
	// * The list of all supported protocol versions
	// * All protocol features and what protocol versions support them
	let info = match build::parse_info_file("protocol.ron") {
		Ok(info) => info,
		Err(e) => panic!("Error while parsing protocol.ron: {e}",),
	};

	// And then parse the packet specifications and generate rust code for them
	build::generate_packets(info);
}
