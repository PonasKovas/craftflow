#[allow(unused_imports)]
use craftflow_protocol_core::datatypes::*;
#[allow(unused_imports)]
use craftflow_protocol_core::*;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct SuccessV00759 {
	pub uuid: u128,
	pub username: String,
	pub properties: Vec<Property>,
}
#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct Property {
	pub name: String,
	pub value: String,
	pub signature: Option<String>,
}
impl MCPWrite for SuccessV00759 {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written_bytes = 0;

		written_bytes += self.uuid.write(output)?;
		written_bytes += self.username.write(output)?;
		written_bytes += (self.properties.len() as i32).write(output)?;
		for property in &self.properties {
			written_bytes += property.name.write(output)?;
			written_bytes += property.value.write(output)?;
			if let Some(ref signature) = property.signature {
				written_bytes += signature.write(output)?;
			} else {
				written_bytes += (-1i32).write(output)?; // Write -1 for None
			}
		}

		Ok(written_bytes)
	}
}
impl MCPRead for SuccessV00759 {
	fn read(input: &mut [u8]) -> Result<(&mut [u8], Self)> {
		let (input, uuid) = u128::read(input)?;
		let (input, username) = String::read(input)?;
		let (mut input, properties_len) = VarInt::read(input)?;
		let mut properties = Vec::new();
		for _ in 0..properties_len.0 {
			let (i, name) = String::read(input)?;
			let (i, value) = String::read(i)?;
			let (i, signature) = Option::<String>::read(i)?;
			properties.push(Property {
				name,
				value,
				signature,
			});
			input = i;
		}

		Ok((
			input,
			Self {
				uuid,
				username,
				properties,
			},
		))
	}
}

impl crate::IntoVersionEnum for SuccessV00759 {
	type Packet = super::super::Success;

	fn into_version_enum(self) -> Self::Packet {
		super::super::Success::V00759(self)
	}
}
impl crate::IntoPacketEnum for SuccessV00759 {
	type State = super::super::super::Login;

	fn into_packet_enum(self) -> Self::State {
		let packet = crate::IntoVersionEnum::into_version_enum(self);
		super::super::super::Login::Success(packet)
	}
}
impl crate::IntoStateEnum for SuccessV00759 {
	type Direction = super::super::super::super::S2C;

	fn into_state_enum(self) -> Self::Direction {
		let state = crate::IntoPacketEnum::into_packet_enum(self);
		super::super::super::super::S2C::Login(state)
	}
}
