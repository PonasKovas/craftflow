
use craftflow_protocol_core::datatypes::*;
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq)]
pub struct SetProtocol {
	pub protocol_version: VarInt,
	pub server_host: String,
	pub server_port: u16,
	pub next_state: VarInt,
}

impl MCPWrite for SetProtocol {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.protocol_version.write(output)?;
		written_bytes += self.server_host.write(output)?;
		written_bytes += self.server_port.write(output)?;
		written_bytes += self.next_state.write(output)?;

		Ok(written_bytes)
	}
}

impl MCPRead for SetProtocol {
	fn read(input: &[u8]) -> Result<(&[u8], Self)> {
		let (input, protocol_version) = VarInt::read(input)?;
		let (input, server_host) = String::read(input)?;
		let (input, server_port) = u16::read(input)?;
		let (input, next_state) = VarInt::read(input)?;

		Ok((
			input,
			Self {
				protocol_version,
				server_host,
				server_port,
				next_state,
			},
		))
	}
}

impl crate::EqvPacket<SetProtocol> for SetProtocol {
	fn into_eqv_packet(self) -> SetProtocol {
		self
	}
	fn from_eqv_packet(p: SetProtocol) -> Self {
		p
	}
}

impl crate::Packet for SetProtocol {
	type Direction = crate::C2S;
	type Version = crate::v00005::C2S;
	type State = crate::v00005::c2s::Handshaking;

	fn into_state_enum(self) -> Self::State {
		crate::v00005::c2s::Handshaking::SetProtocol(self)
	}
	fn into_version_enum(self) -> Self::Version {
		crate::v00005::C2S::Handshaking(self.into_state_enum())
	}
	fn into_direction_enum(self) -> Self::Direction {
		crate::C2S::V00005(self.into_version_enum())
	}
}

impl crate::PacketVersion for SetProtocol {
	const VERSIONS: &'static [u32] = &[
		5, 47, 107, 109, 110, 210, 315, 335, 338, 340, 393, 401, 404, 477, 490, 498, 573, 735, 751,
		755, 756, 757, 758, 759, 760, 761, 762, 763, 764, 765,
	];
}
