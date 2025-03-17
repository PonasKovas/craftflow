/// Automatically implements MCPRead, MCPWrite traits
/// Requires that all fields implement MCPRead, MCPWrite
macro_rules! define_type {
	(
        $(#[$attr:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        pub struct $name {
            $(
            $(#[$field_attr])*
            pub $field_name: $field_type,
            )*
        }

        impl<'read> MCPRead<'read> for $name {
        	fn mcp_read(input: &mut &'read [u8]) -> Result<Self> {
                $(
                    let $field_name = MCPRead::mcp_read(input)?;
                )*

        		Ok(Self {
       				$(
                        $field_name,
                    )*
        		})
        	}
        }

        impl MCPWrite for $name {
            fn mcp_write(&self, #[allow(unused_variables)] output: &mut Vec<u8>) -> usize {
                #[allow(unused_mut)]
                let mut written_bytes = 0;

                $(
                    written_bytes += self.$field_name.mcp_write(output);
                )*

                written_bytes
            }
        }
    };
}
