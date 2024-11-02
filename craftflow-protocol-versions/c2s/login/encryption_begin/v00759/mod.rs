//////////////////////////////////////////////////////////////////////////////////////
// GENERATED // MINECRAFT-DATA COMMIT HASH f1130aea931b948d2ecaecf34ecfe0116bfd4172 //
//////////////////////////////////////////////////////////////////////////////////////

// [
//     "container",
//     [
//         {
//             "name": "sharedSecret",
//             "type": [
//                 "buffer",
//                 {
//                     "countType": "varint"
//                 }
//             ]
//         },
//         {
//             "name": "hasVerifyToken",
//             "type": "bool"
//         },
//         {
//             "name": "crypto",
//             "type": [
//                 "switch",
//                 {
//                     "compareTo": "hasVerifyToken",
//                     "fields": {
//                         "true": [
//                             "container",
//                             [
//                                 {
//                                     "name": "verifyToken",
//                                     "type": [
//                                         "buffer",
//                                         {
//                                             "countType": "varint"
//                                         }
//                                     ]
//                                 }
//                             ]
//                         ],
//                         "false": [
//                             "container",
//                             [
//                                 {
//                                     "name": "salt",
//                                     "type": "i64"
//                                 },
//                                 {
//                                     "name": "messageSignature",
//                                     "type": [
//                                         "buffer",
//                                         {
//                                             "countType": "varint"
//                                         }
//                                     ]
//                                 }
//                             ]
//                         ]
//                     }
//                 }
//             ]
//         }
//     ]
// ]

define_type! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct EncryptionBeginV00759<'a> {
		pub shared_secret: Buffer<'a, VarInt>,
		pub crypto: Crypto<'a>,
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Crypto<'a> {
	VerifyToken {
		verify_token: Buffer<'a, VarInt>,
	},
	SaltAndSignature {
		salt: i64,
		message_signature: Buffer<'a, VarInt>,
	},
}

impl<'a> MCPWrite for Crypto<'a> {
	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
		let mut written = 0;

		match self {
			Crypto::VerifyToken { verify_token } => {
				written += true.write(output)?;
				written += verify_token.write(output)?;
			}
			Crypto::SaltAndSignature {
				salt,
				message_signature,
			} => {
				written += false.write(output)?;
				written += salt.write(output)?;
				written += message_signature.write(output)?;
			}
		}

		Ok(written)
	}
}

impl<'a> MCPRead<'a> for Crypto<'a> {
	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
		let (input, has_verify_token) = bool::read(input)?;

		if has_verify_token {
			let (input, verify_token) = Buffer::read(input)?;
			Ok((input, Crypto::VerifyToken { verify_token }))
		} else {
			let (input, salt) = i64::read(input)?;
			let (input, message_signature) = Buffer::read(input)?;
			Ok((
				input,
				Crypto::SaltAndSignature {
					salt,
					message_signature,
				},
			))
		}
	}
}
