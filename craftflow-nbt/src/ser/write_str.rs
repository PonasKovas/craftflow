use crate::Result;
use std::io::Write;

pub fn write_str<W: Write>(mut output: W, s: &str) -> Result<usize> {
	let mut written = 0;

	let converted = cesu8::to_java_cesu8(s);
	output.write_all(&(converted.len() as u16).to_be_bytes())?;
	written += 2;
	output.write_all(&converted)?;
	written += converted.len();

	Ok(written)
}
