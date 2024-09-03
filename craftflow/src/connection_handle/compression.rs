use tokio::sync::oneshot;

/// Allows the connection writer/reader to enable compression from outside through a channel
pub(crate) enum Compression {
	Disabled { threshold: oneshot::Receiver<usize> },
	Enabled { threshold: usize },
}

impl Compression {
	/// Creates a new compression state with the shared secret receiver
	pub(crate) fn new(threshold: oneshot::Receiver<usize>) -> Self {
		Compression::Disabled { threshold }
	}

	/// Checks the compression threshold if enabled
	pub(crate) fn enabled(&mut self) -> Option<usize> {
		if let Compression::Disabled { threshold } = self {
			// check if threshold arrived
			if let Ok(threshold) = threshold.try_recv() {
				*self = Compression::Enabled { threshold };
			}
		}

		if let Compression::Enabled { threshold } = self {
			Some(*threshold)
		} else {
			None
		}
	}
}
