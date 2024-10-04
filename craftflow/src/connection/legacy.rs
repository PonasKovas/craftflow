mod response;

use crate::reactor::Event;
use anyhow::bail;
use std::time::Duration;
use tokio::{
	io::AsyncWriteExt,
	net::tcp::{OwnedReadHalf, OwnedWriteHalf},
	time::{sleep, timeout},
};

pub use response::LegacyPingResponse;

#[derive(PartialEq, Debug)]
pub enum LegacyPingFormat {
	Pre1_4, // Beta 1.8 to 1.3
	Pre1_6, // 1.4 to 1.5
	Pre1_7, // 1.6
}

/// This is a special packet with a different format that is sent by old clients to ping the server
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LegacyPing;

impl Event for LegacyPing {
	/// Connection ID
	type Args<'a> = u64;
	/// The response to the legacy ping. `None` if should be ignored.
	type Return = Option<LegacyPingResponse>;
}

/// Returns true if legacy ping detected
pub(crate) async fn detect_legacy_ping(
	stream: &mut OwnedReadHalf,
) -> anyhow::Result<Option<LegacyPingFormat>> {
	let mut temp_buf = [0_u8; 3];
	let mut n = match timeout(Duration::from_secs(5), stream.peek(&mut temp_buf)).await {
		Ok(n) => n?,
		Err(_) => bail!("timed out"),
	};

	if let [0xfe] | [0xfe, 0x01] = &temp_buf[..n] {
		// This could mean one of following things:
		// 1. The beginning of a normal handshake packet, not fully received yet though
		// 2. The beginning of the 1.6 legacy ping, not fully received yet either
		// 3. Pre-1.4 legacy ping (0xfe) or 1.4-1.5 legacy ping (0xfe 0x01), fully
		//    received
		//
		// So in the name of the Father, the Son, and the Holy Spirit, we pray,
		// and wait for more data to arrive if it's 1 or 2, and if no
		// data arrives for long enough, we can assume its 3.
		//
		// Downsides of this approach and where this could go wrong:
		// 1. Short artificial delay for pre-1.4 and 1.4-1.5 legacy pings
		// 2. If a normal handshake is encountered with the exact length of 0xfe 0x01 in
		//    VarInt format (extremely rare, the server address would have to be ~248
		//    bytes long), and for some God-forsaken reason sent the first 2 bytes of
		//    the packet but not any more in this whole time, we would incorrectly
		//    assume that it's a legacy ping and send an incorrect response.
		// 3. If it was a 1.6 legacy ping, but even after the delay we only received
		//    only 1 byte, then we would also send an incorrect response, thinking its a
		//    pre-1.4 ping. The client would still understand it though, it'd just think
		//    that the server is old (pre-1.4).
		//
		// 1 is insignificant, and 2/3 are so rare that they are effectively non existant.
		sleep(Duration::from_millis(50)).await;
		n = stream.peek(&mut temp_buf).await?;
	}

	let format = match &temp_buf[..n] {
		[0xfe] => LegacyPingFormat::Pre1_4,
		[0xfe, 0x01] => LegacyPingFormat::Pre1_6,
		[0xfe, 0x01, 0xfa] => LegacyPingFormat::Pre1_7,
		_ => return Ok(None), // Not a legacy ping
	};

	// Pre1_7 has some payload but we don't really care about it :/

	Ok(Some(format))
}

pub(crate) async fn write_legacy_response(
	stream: &mut OwnedWriteHalf,
	format: LegacyPingFormat,
	mut response: LegacyPingResponse,
) -> anyhow::Result<()> {
	if format == LegacyPingFormat::Pre1_4 {
		// remove formatting for pre-1.4 legacy pings
		remove_formatting(&mut response.description);
	}

	let separator = match format {
		LegacyPingFormat::Pre1_4 => '§',
		_ => '\0',
	};

	let mut buf = Vec::new();

	// packet ID and length placeholder
	buf.extend([0xff, 0x00, 0x00]);

	if format != LegacyPingFormat::Pre1_4 {
		// some constant bytes lol
		buf.extend("§1\0".encode_utf16().flat_map(|c| c.to_be_bytes()));

		// protocol and version
		buf.extend(
			format!(
				"{protocol}{separator}{version}{separator}",
				protocol = response.protocol_version,
				version = response.version
			)
			.encode_utf16()
			.flat_map(|c| c.to_be_bytes()),
		);
	}

	// Description
	buf.extend(
		response
			.description
			.encode_utf16()
			.flat_map(|c| c.to_be_bytes()),
	);

	// Online and max players
	buf.extend(
		format!(
			"{separator}{online_players}{separator}{max_players}",
			online_players = response.online_players,
			max_players = response.max_players
		)
		.encode_utf16()
		.flat_map(|c| c.to_be_bytes()),
	);

	// replace the length placeholder with the actual length
	let chars = (buf.len() as u16 - 3) / 2; // -3 because of the packet prefix (id and length), and /2 because UTF16
	buf[1..3].copy_from_slice(chars.to_be_bytes().as_slice());

	stream.write_all(&buf).await?;

	// Must not close the connection instantly otherwise the client will shit itself and show an error
	sleep(Duration::from_secs(1)).await;

	Ok(())
}

// Removes all `§` and their modifiers, if any
fn remove_formatting(string: &mut String) {
	while let Some(pos) = string.find('§') {
		// + 2 because we know that `§` is 2 bytes
		if let Some(c) = string[(pos + 2)..].chars().next() {
			// remove next char too if any
			string.replace_range(pos..(pos + 2 + c.len_utf8()), "");
		} else {
			string.remove(pos);
		}
	}
}
