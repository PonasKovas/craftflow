
pub struct SetProtocol(pub crate::v00005::c2s::handshaking::set_protocol::SetProtocol);

impl crate::EqvPacket<crate::v00005::c2s::handshaking::set_protocol::SetProtocol> for SetProtocol {
	fn into_eqv_packet(self) -> crate::v00005::c2s::handshaking::set_protocol::SetProtocol {
		self.0
	}
	fn from_eqv_packet(p: crate::v00005::c2s::handshaking::set_protocol::SetProtocol) -> Self {
		Self(p)
	}
}

impl std::ops::Deref for SetProtocol {
	type Target = crate::v00005::c2s::handshaking::set_protocol::SetProtocol;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl crate::Packet for SetProtocol {
	type Direction = crate::C2S;
	type Version = crate::v00757::C2S;
	type State = crate::v00757::c2s::Handshaking;

	fn into_state_enum(self) -> Self::State {
		crate::v00757::c2s::Handshaking::SetProtocol(self)
	}
	fn into_version_enum(self) -> Self::Version {
		crate::v00757::C2S::Handshaking(self.into_state_enum())
	}
	fn into_direction_enum(self) -> Self::Direction {
		crate::C2S::V00757(self.into_version_enum())
	}
}

impl crate::PacketVersion for SetProtocol {
	const VERSIONS: &'static [u32] = &[
		5, 47, 107, 109, 110, 210, 315, 335, 338, 340, 393, 401, 404, 477, 490, 498, 573, 735, 751,
		755, 756, 757, 758, 759, 760, 761, 762, 763, 764, 765,
	];
}
