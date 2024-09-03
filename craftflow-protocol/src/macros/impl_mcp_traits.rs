/// Given a group name and a list of packet definitions, this macro will generate
/// the necessary MCPReadable or MCPWritable implementations for each packet.
/// Group name should include the state of the connection where the packets
/// are used (e.g. Handshake, Status, Login, Play) and the direction of the packet (e.g. C2S, S2C).
///
/// It will also generate an enum for the group that will contain all the packet types.
///
/// Usage:
/// ```ignore
/// impl_mcp_traits! {
/// 	<S2C|C2S>: GroupNameDirection;
/// 	[0] PacketName {
/// 		packet_field: FieldType,
/// 	},
/// 	[1] AnotherPacket {},
/// }
/// ```
macro_rules! impl_mcp_traits {
	(S2C: $group:ident; $([$id:literal] $packet_name:ident { $( $field:ident : $field_type:ty ),* $(,)? } )*) => {
		#[repr(u32)]
		#[derive(::std::fmt::Debug, ::std::clone::Clone)]
		pub enum $group {
			$(
				$packet_name ( $packet_name ) = $id,
			)*
		}

		impl ::std::convert::Into<crate::packets::PacketS2C> for $group {
			fn into(self) -> crate::packets::PacketS2C {
				crate::packets::PacketS2C::$group(self)
			}
		}

		impl crate::MCPWritable for $group {
			fn write(&self, to: &mut impl ::std::io::Write) -> ::anyhow::Result<usize> {
				let mut written = 0;

				match self {
					$(
						Self::$packet_name(packet) => {
							written += crate::MCPWritable::write(&crate::datatypes::VarInt($id), to)?;
							written += crate::MCPWritable::write(packet, to)?;
						}
					)*
				}

				Ok(written)
			}
		}

		$(
			#[derive(std::fmt::Debug, ::std::clone::Clone)]
			pub struct $packet_name {
				$(
					pub $field: $field_type,
				)*
			}

			impl crate::MCPWritable for $packet_name {
				fn write(&self, to: &mut impl ::std::io::Write) -> ::anyhow::Result<usize> {
					let mut written = 0;
					$(
						written += crate::MCPWritable::write(&self.$field, to)?;
					)*
					Ok(written)
				}
			}

			impl crate::packets::IntoPacketS2C for $packet_name {
				fn into_packet(self) -> crate::packets::PacketS2C {
					crate::packets::PacketS2C::$group($group::$packet_name(self))
				}
			}
		)*
	};
	(C2S: $group:ident; $([$id:literal] $packet_name:ident { $( $field:ident : $field_type:ty ),* $(,)? } )*) => {
		#[repr(u32)]
		#[derive(std::fmt::Debug, ::std::clone::Clone)]
		pub enum $group {
			$(
				$packet_name ( $packet_name ) = $id,
			)*
		}

		impl ::std::convert::Into<crate::packets::PacketC2S> for $group {
			fn into(self) -> crate::packets::PacketC2S {
				crate::packets::PacketC2S::$group(self)
			}
		}

		impl crate::MCPReadable for $group {
			fn read(source: &mut impl ::std::io::Read) -> ::anyhow::Result<Self> {
				let packet_id = <crate::datatypes::VarInt as crate::MCPReadable>::read(source)?.0 as u32;
				match packet_id {
					$(
						$id => Ok(Self::$packet_name(
							::anyhow::Context::with_context(
								<$packet_name as crate::MCPReadable>::read(source), || format!("packed id {}", packet_id)
							)?
						)),
					)*
					_ => ::anyhow::bail!("Unknown packet id: {}", packet_id),
				}
			}
		}

		$(
			#[derive(std::fmt::Debug, ::std::clone::Clone)]
			pub struct $packet_name {
				$(
					pub $field: $field_type,
				)*
			}

			impl crate::MCPReadable for $packet_name {
				fn read(#[allow(unused)] source: &mut impl ::std::io::Read) -> ::anyhow::Result<Self> {
					Ok(
						Self {
							$(
								$field: <$field_type as crate::MCPReadable>::read(source)?,
							)*
						}
					)
				}
			}

			impl crate::packets::IntoPacketC2S for $packet_name {
				fn into_packet(self) -> crate::packets::PacketC2S {
					crate::packets::PacketC2S::$group($group::$packet_name(self))
				}
			}
		)*
	};
}
