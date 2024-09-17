use crate::reactor::Event;

/// This event is triggered when a new connection is established.
pub struct NewConnection;

/// This event is triggered when a connection is closed.
pub struct Disconnect;

impl Event for NewConnection {
	/// The ID of the connection that was established.
	type Args = usize;
	/// If the event is blocked, connection will be closed.
	type Return = ();
}

impl Event for Disconnect {
	/// The ID of the connection that was closed.
	type Args = usize;
	type Return = ();
}
