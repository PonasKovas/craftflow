mod gen;
mod info_file;
mod state_spec;
mod version_bounds;

use info_file::{parse_info_file, Info};
use proc_macro2::TokenStream;
use state_spec::parse_state_spec;
use std::{
	collections::BTreeMap,
	env, fs,
	io::Write,
	path::Path,
	process::{Command, Stdio},
	str::FromStr,
};

pub fn main() {
	println!("cargo::rerun-if-changed=packets/");
	println!("cargo::rerun-if-changed=protocol.toml");

	// First handle the main protocol info file which includes
	// * The list of all supported protocol versions
	// * All protocol features and what protocol versions support them
	let info = match parse_info_file("protocol.ron") {
		Ok(info) => info,
		Err(e) => panic!("Error while parsing protocol.ron: {e}",),
	};

	// And then parse the packet specifications and generate rust code for them
	generate_packets(info);
}

/// Parses the packet specifications and generates rust code for them
pub fn generate_packets(info: Info) {
	// Parse the state specs
	let mut states = [BTreeMap::new(), BTreeMap::new()];

	for (i, direction) in ["c2s", "s2c"].into_iter().enumerate() {
		for state in fs::read_dir(format!("packets/{direction}/")).unwrap() {
			let state = state.unwrap();
			let path = state.path();
			let state_name = path.file_stem().unwrap().to_str().unwrap();

			let state_spec = match parse_state_spec(&path) {
				Ok(state_spec) => state_spec,
				Err(e) => panic!(
					"Error while parsing state specification ({:?}): {}",
					path, e
				),
			};

			states[i].insert(state_name.to_owned(), state_spec);
		}
	}

	// generate the code
	let generated = gen::generate_code(&info, &states[0], &states[1]);

	write(&generated, "generated.rs");
	write(
		&generated,
		Path::new(&env::var("OUT_DIR").unwrap()).join("generated_packets.rs"),
	);
}

trait AsIdent {
	fn as_ident(&self) -> proc_macro2::Ident;
}
impl AsIdent for str {
	fn as_ident(&self) -> proc_macro2::Ident {
		proc_macro2::Ident::new(self, proc_macro2::Span::call_site())
	}
}

trait AsTokenStream {
	fn as_tokenstream(&self) -> proc_macro2::TokenStream;
}
impl AsTokenStream for str {
	fn as_tokenstream(&self) -> proc_macro2::TokenStream {
		proc_macro2::TokenStream::from_str(self).unwrap()
	}
}

fn to_pascal_case(s: &str) -> String {
	fn capitalize(s: &str) -> String {
		let mut c = s.chars();
		match c.next() {
			None => String::new(),
			Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
		}
	}

	s.split('_').map(|word| capitalize(word)).collect()
}

// Writes (formatted if env RUSTFMT_GENERATED set) stream to the given path
fn write(stream: &TokenStream, path: impl AsRef<Path>) {
	let text = if env::var("RUSTFMT_GENERATED").is_ok() {
		let mut rustfmt = Command::new("rustfmt")
			.arg("--edition")
			.arg("2021")
			.arg("--config")
			.arg("max_width=1000")
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
			.unwrap();

		{
			let stdin = rustfmt.stdin.as_mut().unwrap();
			stdin.write_all(format!("{stream}").as_bytes()).unwrap();
		}

		let output = rustfmt.wait_with_output().unwrap();

		if !output.status.success() {
			let stderr = String::from_utf8_lossy(&output.stderr);
			eprintln!("rustfmt failed:\n{}", stderr);
			panic!("rustfmt encountered an error");
		}

		String::from_utf8(output.stdout).unwrap()
	} else {
		format!("{stream}")
	};

	fs::write(path, text).unwrap();
}
