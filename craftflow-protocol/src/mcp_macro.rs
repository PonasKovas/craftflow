/// Automatically implements MCPRead, MCPWrite traits
/// Requires that all fields implement MCPRead, MCPWrite
macro_rules! mcp {
	(
        $(#[$attr:meta])*
        pub struct $name:ident $({
            $(
                $(#[$field_attr:meta])*
                // because of macro hygiene slop we cant just match a `ty` here because then we cant replace VarInts with i32s etc
                // so we have to do this hack
                // basically shitty pretend `ty`
                //
                // pub $field_name:ident: $field_type:ty
                pub $field_name:ident: $field_type:ident $(<$($field_type_generics:tt),*>)?
            ),* $(,)?
        })? $(;)?
    ) => {
        $(#[$attr])*
        pub struct $name $({
            $(
            $(#[$field_attr])*
            pub $field_name: _mcp_field_type!($field_type $(<$($field_type_generics,)*>)?),
            )*
        }
        // funny hack to make the semicolon work when struct has fields (you cant normally type ; after {})
        const _:() = ())?;

        impl<'read> MCPRead<'read> for $name {
        	fn mcp_read(#[allow(unused_variables)] input: &mut &'read [u8]) -> Result<Self> {
                $($(
                    let $field_name = _mcp_read_field!(input, $field_type $(<$($field_type_generics,)*>)?);
                )*)?

        		Ok(Self $({
       				$(
                        $field_name,
                    )*
        		})?)
        	}
        }

        impl MCPWrite for $name {
            fn mcp_write(&self, #[allow(unused_variables)] output: &mut Vec<u8>) -> usize {
                #[allow(unused_mut)]
                let mut written_bytes = 0;

                $($(
                    written_bytes += _mcp_write_field!(output, self.$field_name, $field_type $(<$($field_type_generics,)*>)?);
                )*)?

                written_bytes
            }
        }
    };
}

macro_rules! _mcp_field_type {
    (VarInt) => { i32 };
    (VarLong) => { i64 };
    ($other:ty) => { $other };
}

macro_rules! _mcp_read_field {
    ($input:expr, VarInt) => {{ let varint = VarInt::mcp_read($input)?; varint.0 }};
    ($input:expr, VarLong) => {{ let varlong = VarLong::mcp_read($input)?; varlong.0 }};
    ($input:expr, $other:ty) => { <$other as MCPRead>::mcp_read($input)? };
}

macro_rules! _mcp_write_field {
    ($output:expr, $field:expr, VarInt) => { VarInt($field).mcp_write($output) };
    ($output:expr, $field:expr, VarLong) => { VarLong($field).mcp_write($output) };
    ($output:expr, $field:expr, $other:ty) => { <$other as MCPWrite>::mcp_write(&$field, $output) };
}