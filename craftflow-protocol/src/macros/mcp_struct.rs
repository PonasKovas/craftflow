macro_rules! mcp_struct {
	($packet_name:ident { $( $field:ident : $field_type:ty ),* $(,)? } ) => {
		#[derive(std::fmt::Debug, ::std::clone::Clone)]
		pub struct $packet_name {
			$(
				pub $field: $field_type,
			)*
		}

		impl crate::MCPWritable for $packet_name {
			fn write(&self, to: &mut impl ::std::io::Write) -> ::anyhow::Result<usize> {
				let mut written = 0;
				$(
					written += crate::MCPWritable::write(&self.$field, to)?;
				)*
				Ok(written)
			}
		}

		impl crate::MCPReadable for $packet_name {
			fn read(#[allow(unused)] source: &mut impl ::std::io::Read) -> ::anyhow::Result<Self> {
				Ok(
					Self {
						$(
							$field: <$field_type as crate::MCPReadable>::read(source)?,
						)*
					}
				)
			}
		}
	};
}
