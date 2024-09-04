//! Allows to enable compression in the connection writer/reader tasks from outside through a channel
//!

use std::sync::Mutex;
use tokio::sync::oneshot;

/// The outside setter for the compression threshold
pub(crate) struct CompressionSetter {
	sender: Mutex<Option<[oneshot::Sender<usize>; 2]>>,
}

/// The inside getter for the compression threshold for the reader and writer tasks
pub(crate) enum CompressionGetter {
	Disabled { receiver: oneshot::Receiver<usize> },
	Enabled { threshold: usize },
}

/// Creates a new compression state with one setter and two getters
pub(crate) fn new() -> (CompressionSetter, CompressionGetter, CompressionGetter) {
	let (sender1, receiver1) = oneshot::channel();
	let (sender2, receiver2) = oneshot::channel();

	(
		CompressionSetter {
			sender: Mutex::new(Some([sender1, sender2])),
		},
		CompressionGetter::Disabled {
			receiver: receiver1,
		},
		CompressionGetter::Disabled {
			receiver: receiver2,
		},
	)
}

impl CompressionSetter {
	/// Sets the compression threshold
	/// Returns Ok if successful, Err if the threshold was already set
	pub(crate) fn set(&self, threshold: usize) -> Result<(), ()> {
		match self.sender.lock().unwrap().take() {
			Some([sender1, sender2]) => {
				let _ = sender1.send(threshold);
				let _ = sender2.send(threshold);
				Ok(())
			}
			None => Err(()),
		}
	}
}

impl CompressionGetter {
	/// Checks the compression threshold if enabled
	pub(crate) fn enabled(&mut self) -> Option<usize> {
		if let CompressionGetter::Disabled { receiver } = self {
			// check if threshold arrived
			if let Ok(threshold) = receiver.try_recv() {
				*self = CompressionGetter::Enabled { threshold };
			}
		}

		if let CompressionGetter::Enabled { threshold } = self {
			Some(*threshold)
		} else {
			None
		}
	}
}
