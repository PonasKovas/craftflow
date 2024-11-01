/// Automatically implements MCPRead, MCPWrite traits
/// Requires that all fields implement MCPRead, MCPWrite
/// If using a lifetime, it must be 'a, no other generics or lifetimes allowed
macro_rules! define_type {
	(
        $(#[$attr:meta])*
        pub struct $name:ident $( <$lifetime:lifetime> )? {
            $(
                $(#[$field_attr:meta])*
                pub $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        pub struct $name $( <$lifetime> )? {
            $(
                $(#[$field_attr])*
                pub $field_name: $field_type,
            )*
        }

        impl $( <$lifetime> )? MCPWrite for $name $( <$lifetime> )? {
        	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
        		let mut written_bytes = 0;

                $(
                    written_bytes += self.$field_name.write(output)?;
                )*

        		Ok(written_bytes)
        	}
        }
        impl<'a> MCPRead<'a> for $name $( <$lifetime> )? {
        	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
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
	// Simple enums with no fields
	// More complex enums will need manual implementations
	(
	    tag $tag_type:ty;  // the type that will be used to encode the tag, must implement MCPRead, MCPWrite
					       // and also Self: TryInto<i32> and i32: TryInto<Self>

        $(#[$attr:meta])*
        pub enum $name:ident $( <$lifetime:lifetime> )? {
            $(
                $(#[$field_attr:meta])*
                $variant_name:ident = $variant_value:expr
            ),* $(,)?
        }
    ) => {
        $(#[$attr])*
        pub enum $name $( <$lifetime> )? {
            $(
                $(#[$field_attr])*
                $variant_name = $variant_value,
            )*
        }

        impl $( <$lifetime> )? MCPWrite for $name $( <$lifetime> )? {
        	fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
                match self {
                    $(
                        Self::$variant_name => {
                            $variant_value.try_into::<$tag_type>()
                                .map_err(Error::InvalidData(
                                    format!("cant convert enum tag {} to {}", $variant_value, stringify!($tag_type))
                                ))
                                .write(output)
                        }
                    )*
                }

        	}
        }
        impl<'a> MCPRead<'a> for $name $( <$lifetime> )? {
        	fn read(input: &'a [u8]) -> Result<(&'a [u8], Self)> {
                let (input, tag) = $tag_type::read(input)?;
                let tag: i32 = tag.try_into().map_err(Error::InvalidData(
                    format!("cant convert enum tag {tag} to i32")
                ));

                match tag {
                    $(
                        $variant_value => Ok((input, Self::$variant_name)),
                    )*
                    _ => Err(Error::InvalidData(format!("unknown enum tag {tag}"))),
                }
        	}
        }
    };
}
