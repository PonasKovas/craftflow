/// Marker trait for packets
pub trait IsPacket {}

// /// Convenience trait for converting a packet from any state into a general packet enum
// pub trait IntoPacketC2S {
// 	fn into_packet(self) -> PacketC2S;
// }
// /// Convenience trait for converting a packet from any state into a general packet enum
// pub trait IntoPacketS2C {
// 	fn into_packet(self) -> PacketS2C;
// }

include!(concat!(env!("OUT_DIR"), "/generated_packets.rs"));
