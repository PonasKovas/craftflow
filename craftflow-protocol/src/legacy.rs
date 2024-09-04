use crate::packets::IntoPacketC2S;

// This is a special packet with a different format sent by old clients
#[derive(Debug)]
pub struct LegacyPing;

/// This is a special packet with a different format that is sent in response to a legacy ping
/// Do not edit the fields directly, use the builder methods instead, otherwise might end up with an invalid packet
#[derive(Debug)]
pub struct LegacyPingResponse {
	pub protocol_version: i32,
	pub version: String,
	pub online_players: i32,
	pub max_players: i32,
	pub description: String,
}

impl IntoPacketC2S for LegacyPing {
	fn into_packet(self) -> super::PacketC2S {
		super::PacketC2S::Legacy(self)
	}
}

impl IntoPacketS2C for LegacyPingResponse {
	fn into_packet(self) -> super::PacketS2C {
		super::PacketS2C::Legacy(self)
	}
}

impl IsPacket for LegacyPing {}
impl IsPacket for LegacyPingResponse {}

impl LegacyPingResponse {
	const MAX_VALID_LENGTH: usize = 248;

	// Length of all the fields combined in string form. Used for validating and
	// comparing with MAX_VALID_LENGTH.
	fn length(&self) -> usize {
		let mut len = 0;
		len += int_len(self.protocol_version);
		len += int_len(self.online_players);
		len += int_len(self.max_players);
		len += self.version.encode_utf16().count();
		len += self.description.encode_utf16().count();

		len
	}
	/// Constructs a new basic [`ServerListLegacyPingResponse`].
	///
	/// See [`description`][Self::description] and [`version`][Self::version].
	pub fn new(protocol_version: i32, online_players: i32, max_players: i32) -> Self {
		Self {
			protocol_version,
			version: String::new(),
			online_players,
			max_players,
			description: String::new(),
		}
	}
	/// Sets the description of the server.
	///
	/// If the resulting response packet is too long to be valid, the
	/// description will be truncated.
	///
	/// Use [`max_description`][Self::max_description] method to get the max
	/// valid length for this specific packet with the already set fields
	/// (version, protocol, online players, max players).
	pub fn set_description(mut self, description: String) -> Self {
		self.description = description;

		let overflow = self.length() as i32 - Self::MAX_VALID_LENGTH as i32;
		if overflow > 0 {
			let truncation_index = self
				.description
				.char_indices()
				.nth(self.description.encode_utf16().count() - overflow as usize)
				.unwrap()
				.0;
			self.description.truncate(truncation_index);
		}

		self
	}
	/// Sets the version of the server.
	///
	/// If the resulting response packet is too long to be valid, the
	/// version will be truncated.
	///
	/// Use [`max_version`][Self::max_version] method to get the max valid
	/// length for this specific packet with the already set fields
	/// (description, protocol, online players, max players).
	pub fn set_version(mut self, version: String) -> Self {
		self.version = version;

		let overflow = self.length() as i32 - Self::MAX_VALID_LENGTH as i32;
		if overflow > 0 {
			let truncation_index = self
				.version
				.char_indices()
				.nth(self.version.encode_utf16().count() - overflow as usize)
				.unwrap()
				.0;
			self.version.truncate(truncation_index);
		}

		self
	}
	/// Returns the maximum number of characters (not bytes) that this packet's
	/// description can have with all other fields set as they are.
	pub fn max_description(&self) -> usize {
		Self::MAX_VALID_LENGTH - (self.length() - self.description.encode_utf16().count())
	}
	/// Returns the maximum number of characters (not bytes) that this packet's
	/// version can have with all other fields set as they are.
	pub fn max_version(&self) -> usize {
		Self::MAX_VALID_LENGTH - (self.length() - self.version.encode_utf16().count())
	}
}

// Returns the length of a string representation of a signed integer
fn int_len(num: i32) -> usize {
	let num_abs = f64::from(num.abs());

	if num < 0 {
		(num_abs.log10() + 2.0) as usize // because minus sign
	} else {
		(num_abs.log10() + 1.0) as usize
	}
}
