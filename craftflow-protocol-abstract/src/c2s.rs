mod handshake;

use handshake::AbHandshake;

/// All packets that can be sent from the client to the server
pub enum AbC2S {
	// There is only one packet in the Handshaking state and its identical in all protocol versions
	Handshake(AbHandshake),
}
