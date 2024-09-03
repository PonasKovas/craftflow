use aes::cipher::{consts::U16, IvSizeUser, KeyIvInit, KeySizeUser};
use tokio::sync::oneshot;

pub(crate) type Encryptor = EncryptionState<cfb8::Encryptor<aes::Aes128>>;
pub(crate) type Decryptor = EncryptionState<cfb8::Decryptor<aes::Aes128>>;

/// Allows the connection writer/reader to enable encryption from outside through a channel
pub(crate) enum EncryptionState<E> {
	Disabled {
		shared_secret: oneshot::Receiver<[u8; 16]>,
	},
	Enabled {
		encryptor: E,
	},
}

impl<E> EncryptionState<E> {
	/// Creates a new encryption state with the shared secret receiver
	pub(crate) fn new(shared_secret: oneshot::Receiver<[u8; 16]>) -> Self {
		EncryptionState::Disabled { shared_secret }
	}
}

impl<E: KeyIvInit + KeySizeUser<KeySize = U16> + IvSizeUser<IvSize = U16>> EncryptionState<E> {
	/// If encryption is enabled runs the closure with the encryptor
	/// Checks if shared secret received, in which case it initialises the encryptor
	pub(crate) fn if_enabled<F: FnOnce(&mut E)>(&mut self, f: F) {
		if let EncryptionState::Disabled { shared_secret } = self {
			// check if secret arrived
			if let Ok(secret) = shared_secret.try_recv() {
				let enc = E::new(&secret.into(), &secret.into());
				*self = EncryptionState::Enabled { encryptor: enc };
			}
		}

		if let EncryptionState::Enabled { encryptor } = self {
			f(encryptor);
		}
	}
}
