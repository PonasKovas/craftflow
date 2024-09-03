/// This generates an enum with variants that have a specific ID.
/// The enum is represented as a `i32` and can be read/written in the Minecraft protocol format.
macro_rules! varint_enum {
	( $name:ident { $( $variant:ident = $id:literal ),+ $(,)? } ) => {
		#[repr(i32)]
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum $name {
			$(
				$variant = $id,
			)+
		}

		impl $name {
			pub fn from_id(id: i32) -> Option<Self> {
				match id {
					$(
						$id => Some(Self::$variant),
					)+
					_ => None,
				}
			}

			pub fn id(&self) -> i32 {
				match self {
					$(
						Self::$variant => $id,
					)+
				}
			}
		}

		impl ::std::ops::Deref for $name {
			type Target = i32;

			fn deref(&self) -> &Self::Target {
				// SAFETY: The enum is repr(i32) so the reference is valid
				unsafe { (self as *const Self as *const i32).as_ref().unwrap_unchecked() }
			}
		}

		impl crate::MCPReadable for $name {
			fn read(source: &mut impl ::std::io::Read) -> ::anyhow::Result<Self> {
				let id = VarInt::read(source)?.0;
				match Self::from_id(id) {
					Some(state) => Ok(state),
					None => ::anyhow::bail!(
						"{} is not a valid ID for the {:?} VarInt enum (valid IDs: {:?})",
						id,
						::std::stringify!($name),
						&[$($id),+]
					)
				}
			}
		}

		impl crate::MCPWritable for $name {
			fn write(&self, to: &mut impl ::std::io::Write) -> ::anyhow::Result<usize> {
				VarInt(**self).write(to)
			}
		}
	};
}
