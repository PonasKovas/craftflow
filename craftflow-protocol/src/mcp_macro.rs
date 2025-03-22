/// Automatically implements MCPRead, MCPWrite traits
/// Requires that all fields implement MCPRead, MCPWrite
macro_rules! mcp {
	(
        $(#[$attr:meta])*
        pub struct $name:ident $({
            $(
                $(#[$field_attr:meta])*
                pub $field_name:ident: $field_type:ty
            ),* $(,)?
        })? $(;)?
    ) => {
        $(#[$attr])*
        pub struct $name
        $(
            {
                $(
                $(#[$field_attr])*
                pub $field_name: <$field_type as MCP>::Data,
                )*
            }
            // funny hack to make the semicolon work when struct has fields (you cant normally type ; after {})
            const _:() = ()
        )?;

        impl MCP for $name {
            type Data = Self;
        }
        impl<'read> MCPRead<'read> for $name {
        	fn mcp_read(#[allow(unused_variables)] input: &mut &'read [u8]) -> Result<Self> {
                $($(
                    let $field_name = <$field_type>::mcp_read(input)?;
                )*)?

        		Ok(Self $({
       				$(
                        $field_name,
                    )*
        		})?)
        	}
        }

        impl MCPWrite for $name {
            fn mcp_write(#[allow(unused_variables)] data: &Self, #[allow(unused_variables)] output: &mut Vec<u8>) -> usize {
                #[allow(unused_mut)]
                let mut written_bytes = 0;

                $($(
                    written_bytes += <$field_type>::mcp_write(&data.$field_name, output);
                )*)?

                written_bytes
            }
        }
    };
}
