use craftflow_protocol_core::*;
use craftflow_protocol_core::datatypes::*;

#[derive(Debug, PartialEq)]
pub struct PacketSetProtocol {
    pub protocol_version: VarInt,
    pub server_host: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl MCPWrite for PacketSetProtocol {
    fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
        let mut written_bytes = 0;

        written_bytes += self.protocol_version.write(output)?;
        written_bytes += self.server_host.write(output)?;
        written_bytes += self.server_port.write(output)?;
        written_bytes += self.next_state.write(output)?;

        Ok(written_bytes)
    }
}

impl MCPRead for PacketSetProtocol {
    fn read(input: &[u8]) -> Result<(&[u8], Self)> {
        let (input, protocol_version) = VarInt::read(input)?;
        let (input, server_host) = String::read(input)?;
        let (input, server_port) = u16::read(input)?;
        let (input, next_state) = VarInt::read(input)?;

        Ok((input, Self { protocol_version, server_host, server_port, next_state }))
    }
}