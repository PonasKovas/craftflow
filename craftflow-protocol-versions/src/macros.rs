/// Automatically implements MCPRead, MCPWrite traits
/// Requires that all fields implement MCPRead, MCPWrite
/// No generics allowed, only a lifetimes
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

        impl MCPWrite for $name {
        	fn write(&self, #[allow(unused_variables)] output: &mut impl std::io::Write) -> Result<usize> {
                #[allow(unused_mut)]
        		let mut written_bytes = 0;

                $(
                    written_bytes += self.$field_name.write(output)?;
                )*

        		Ok(written_bytes)
        	}
        }
        impl<'read> MCPRead<'read> for $name {
        	fn read(input: &'read [u8]) -> Result<(&'read [u8], Self)> {
                $(
                    let (input, $field_name) = MCPRead::read(input)?;
                )*

        		Ok((
        			input,
        			Self {
           				$(
                            $field_name,
                        )*
        			},
        		))
        	}
        }
    };
    (
        $(#[$attr:meta])*
        pub struct $name:ident<'a> {
            $(
                $(#[$field_attr:meta])*
                pub $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
            pub struct $name<'a> {
            $(
                $(#[$field_attr])*
                pub $field_name: $field_type,
            )*
        }

        impl<'a> MCPWrite for $name<'a> {
           	fn write(&self, #[allow(unused_variables)] output: &mut impl std::io::Write) -> Result<usize> {
                    #[allow(unused_mut)]
              		let mut written_bytes = 0;

                    $(
                        written_bytes += self.$field_name.write(output)?;
                    )*

              		Ok(written_bytes)
           	}
        }
        impl<'read> MCPRead<'read> for $name<'read> {
           	fn read(input: &'read [u8]) -> Result<(&'read [u8], Self)> {
                    $(
                        let (input, $field_name) = MCPRead::read(input)?;
                    )*

              		Ok((
             			input,
             			Self {
                				$(
                                $field_name,
                            )*
             			},
              		))
           	}
        }
    };
}
