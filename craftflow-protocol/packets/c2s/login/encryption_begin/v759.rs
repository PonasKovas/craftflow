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
		pub shared_secret: (Buffer),
		pub crypto: (Crypto),
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Crypto {
	WithVerifyToken {
		verify_token: <Buffer as MCP>::Data,
	},
	WithoutVerifyToken {
		salt: <i64 as MCP>::Data,
		message_signature: <Buffer as MCP>::Data,
	},
}

impl MCP for Crypto {
	type Data = Self;
}
impl MCPWrite for Crypto {
	fn mcp_write(data: &Self, output: &mut Vec<u8>) -> usize {
		let mut written_bytes = 0;

		match data {
			Crypto::WithVerifyToken { verify_token } => {
				written_bytes += bool::mcp_write(&true, output);
				written_bytes += <Buffer>::mcp_write(verify_token, output);
			}
			Crypto::WithoutVerifyToken {
				salt,
				message_signature,
			} => {
				written_bytes += bool::mcp_write(&false, output);
				written_bytes += i64::mcp_write(salt, output);
				written_bytes += <Buffer>::mcp_write(message_signature, output);
			}
		}

		written_bytes
	}
}

impl<'a> MCPRead<'a> for Crypto {
	fn mcp_read(input: &mut &'a [u8]) -> Result<Self> {
		let has_verify_token = bool::mcp_read(input)?;

		if has_verify_token {
			let verify_token = <Buffer>::mcp_read(input)?;
			Ok(Self::WithVerifyToken { verify_token })
		} else {
			let salt = i64::mcp_read(input)?;
			let message_signature = <Buffer>::mcp_read(input)?;
			Ok(Self::WithoutVerifyToken {
				salt,
				message_signature,
			})
		}
	}
}
