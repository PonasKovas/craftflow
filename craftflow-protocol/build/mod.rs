mod gen;
mod info_file;
mod state_spec;
mod version_bounds;

pub use info_file::{parse_info_file, Info};

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

// Writes formatted stream to the given path
fn write(stream: &TokenStream, path: impl AsRef<Path>) {
	let mut rustfmt = Command::new("rustfmt")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.unwrap();

	{
		let stdin = rustfmt.stdin.as_mut().unwrap();
		stdin.write_all(format!("{stream}").as_bytes()).unwrap();
	}

	let output = rustfmt.wait_with_output().unwrap();

	fs::write(path, String::from_utf8(output.stdout).unwrap()).unwrap();
}
