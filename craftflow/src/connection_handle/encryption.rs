//! Allows to enable encryption in the connection writer/reader tasks from outside through a channel
//!

use aes::cipher::{consts::U16, IvSizeUser, KeyIvInit, KeySizeUser};
use std::sync::Mutex;
use tokio::sync::oneshot;

/// The outside setter for the encryption shared secret
pub(crate) struct EncryptionSetter {
	sender: Mutex<Option<[oneshot::Sender<[u8; 16]>; 2]>>,
}

/// The inside getter for the encryption shared secret for the writer task
pub(crate) type Encryptor = EncryptionGetter<cfb8::Encryptor<aes::Aes128>>;
/// The inside getter for the encryption shared secret for the reader task
pub(crate) type Decryptor = EncryptionGetter<cfb8::Decryptor<aes::Aes128>>;

/// The inside getter for the encryption state for the reader and writer tasks
/// This is generic over the encryptor/decryptor type (reader uses decryptor, writer uses encryptor)
pub(crate) enum EncryptionGetter<E> {
	Disabled {
		receiver: oneshot::Receiver<[u8; 16]>,
	},
	Enabled {
		encryptor: E,
	},
}

/// Creates a new encryption state with one setter and two getters (encryption and decryption)
pub(crate) fn new() -> (EncryptionSetter, Encryptor, Decryptor) {
	let (sender1, receiver1) = oneshot::channel();
	let (sender2, receiver2) = oneshot::channel();

	(
		EncryptionSetter {
			sender: Mutex::new(Some([sender1, sender2])),
		},
		Encryptor::Disabled {
			receiver: receiver1,
		},
		Decryptor::Disabled {
			receiver: receiver2,
		},
	)
}

impl EncryptionSetter {
	/// Sets the encryption shared secret
	/// Returns Ok if successful, Err if the secret was already set
	pub(crate) fn set(&self, shared_secret: [u8; 16]) -> Result<(), ()> {
		match self.sender.lock().unwrap().take() {
			Some([sender1, sender2]) => {
				let _ = sender1.send(shared_secret);
				let _ = sender2.send(shared_secret);
				Ok(())
			}
			None => Err(()),
		}
	}
}

impl<E: KeyIvInit + KeySizeUser<KeySize = U16> + IvSizeUser<IvSize = U16>> EncryptionGetter<E> {
	/// If encryption is enabled runs the closure with the encryptor
	/// Checks if shared secret received, in which case it initialises the encryptor
	pub(crate) fn if_enabled<F: FnOnce(&mut E)>(&mut self, f: F) {
		if let EncryptionGetter::Disabled { receiver } = self {
			// check if secret arrived
			if let Ok(secret) = receiver.try_recv() {
				let enc = E::new(&secret.into(), &secret.into());
				*self = EncryptionGetter::Enabled { encryptor: enc };
			}
		}

		if let EncryptionGetter::Enabled { encryptor } = self {
			f(encryptor);
		}
	}
}
