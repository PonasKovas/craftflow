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

mcp! {
	#[derive(Debug, PartialEq, Clone)]
	pub struct EncryptionBeginV759 {
		pub shared_secret: Buffer,
		pub crypto: Crypto,
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Crypto {
	WithVerifyToken {
		verify_token: Buffer,
	},
	WithoutVerifyToken {
		salt: i64,
		message_signature: Buffer,
	},
}

impl MCPWrite for Crypto {
	fn mcp_write(&self, output: &mut Vec<u8>) -> usize {
		let mut written_bytes = 0;

		match self {
			Crypto::WithVerifyToken { verify_token } => {
				written_bytes += true.mcp_write(output);
				written_bytes += verify_token.mcp_write(output);
			}
			Crypto::WithoutVerifyToken {
				salt,
				message_signature,
			} => {
				written_bytes += false.mcp_write(output);
				written_bytes += salt.mcp_write(output);
				written_bytes += message_signature.mcp_write(output);
			}
		}

		written_bytes
	}
}

impl<'a> MCPRead<'a> for Crypto {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let has_verify_token = bool::mcp_read(input)?;

		if has_verify_token {
			let verify_token = Buffer::mcp_read(input)?;
			Ok(Self::WithVerifyToken { verify_token })
		} else {
			let salt = i64::mcp_read(input)?;
			let message_signature = Buffer::mcp_read(input)?;
			Ok(Self::WithoutVerifyToken {
				salt,
				message_signature,
			})
		}
	}
}
