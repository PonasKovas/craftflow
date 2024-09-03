use tokio::{
	io::{AsyncRead, AsyncReadExt, BufReader},
	net::TcpStream,
};

pub(crate) async fn read_varint<S: AsyncRead + Unpin>(socket: &mut S) -> std::io::Result<i32> {
	let mut num_read = 0; // Count of bytes that have been read
	let mut result = 0i32; // The VarInt being constructed

	loop {
		// VarInts are at most 5 bytes long.
		if num_read >= 5 {
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"VarInt is too big",
			));
		}

		// Read a byte
		let byte = socket.read_u8().await?;

		// Extract the 7 lower bits (the data bits) and cast to i32
		let value = (byte & 0b0111_1111) as i32;

		// Shift the data bits to the correct position and add them to the result
		result |= value << (7 * num_read);

		num_read += 1;

		// If the high bit is not set, this was the last byte in the VarInt
		if (byte & 0b1000_0000) == 0 {
			break;
		}
	}

	Ok(result)
}
