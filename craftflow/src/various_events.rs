use closureslop::Event;
use craftflow_protocol_core::common_structures::Text;

/// This event is triggered when a new connection is established.
pub struct NewConnection;

/// This event is triggered when a connection is closed.
pub struct Disconnect;

/// This event is triggered when a client tries to connect with an unsupported protocol version.
pub struct UnsupportedClientVersion;

impl Event for NewConnection {
	/// The ID of the connection that was established.
	type Args<'a> = u64;
	/// If the event is blocked, connection will be closed.
	type Return = ();
}

impl Event for Disconnect {
	/// The ID of the connection that was closed.
	type Args<'a> = u64;
	type Return = ();
}

impl Event for UnsupportedClientVersion {
	/// The ID of the connection and the protocol version
	type Args<'a> = (u64, u32);
	/// The error message to send to the client
	/// If no handler returns, a default message will be used
	type Return = Text<'static>;
}
