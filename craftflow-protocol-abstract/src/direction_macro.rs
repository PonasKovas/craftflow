macro_rules! gen_direction_enum {
    (
        @DIRECTION=$direction:ident;
        $( #[$attr:meta] )*
        pub enum $name:ident<'a> {
            $( $variant:ident ( $struct:ident $( <$var_lifetime:lifetime> )? ) ),*
            $(,)?
        }
    ) => {
	    $( #[$attr] )*
		pub enum $name<'a> {
		    $($variant($struct $(<$var_lifetime>)? ), )*
		}

		impl<'a> $name<'a> {
		    /// Returns the str name of the abstract packet for debugging purposes
			// todo this might be unused, remove if it is
		    pub fn variant_name(&self) -> &'static str {
                match self {
                    $( $name::$variant(_) => stringify!($variant), )*
                }
            }
		}

		impl<'a> crate::AbPacketWrite<'a> for $name<'a> {
            type Direction = craftflow_protocol_versions::$direction<'a>;
            type Iter = Box<dyn Iterator<Item = Self::Direction> + Send + Sync + 'a>;

            fn convert(
          		&'a self,
          		protocol_version: u32,
                state: crate::State,
           	) -> anyhow::Result<crate::WriteResult<Self::Iter>> {
                Ok(match self {
                    $(
                        $name::$variant(inner) => match inner.convert(protocol_version, state)? {
                            crate::WriteResult::Success(iter) => crate::WriteResult::Success(Box::new(iter)),
                            crate::WriteResult::Unsupported => crate::WriteResult::Unsupported,
                        },
                    )*
                })
            }
        }

        impl<'a> crate::AbPacketNew<'a> for $name<'a> {
            type Direction = craftflow_protocol_versions::$direction<'a>;
            type Constructor = Box<dyn crate::AbPacketConstructor<'a,
                Direction = Self::Direction,
                AbPacket = Self
            > + Send + Sync + 'a>;

            fn construct(
                packet: &'a Self::Direction,
            ) -> anyhow::Result<crate::ConstructorResult<Self, Self::Constructor>> {
                $(
                    match $struct::construct(packet)? {
                        crate::ConstructorResult::Ignore => {},
                        crate::ConstructorResult::Done(inner) => return Ok(crate::ConstructorResult::Done(Self::$variant(inner))),
                        crate::ConstructorResult::Continue(inner) => {
                            // A constructor wrapper that converts the result to the enum variant
                            struct __ConstructorWrapper<'a>(<$struct $(<$var_lifetime>)? as crate::AbPacketNew<'a>>::Constructor);
                            impl<'b> crate::AbPacketConstructor<'b> for __ConstructorWrapper<'b> {
                                type Direction = craftflow_protocol_versions::$direction<'b>;
                                type AbPacket = $name<'b>;

                                fn next_packet(
                                    &mut self,
                              		packet: &'b Self::Direction,
                               	) -> anyhow::Result<crate::ConstructorResult<Self::AbPacket, ()>> {
                                    Ok(match self.0.next_packet(packet)? {
                                        crate::ConstructorResult::Done(pkt) =>
                                            crate::ConstructorResult::Done($name::$variant(pkt)),
                                        crate::ConstructorResult::Continue(()) =>
                                            crate::ConstructorResult::Continue(()),
                                        crate::ConstructorResult::Ignore => crate::ConstructorResult::Ignore,
                                    })
                                }
                            }

                            return Ok(crate::ConstructorResult::Continue(Box::new(__ConstructorWrapper(inner))))},
                    }
                )*

                Ok(crate::ConstructorResult::Ignore)
            }
        }

        $(
            impl<'a> From<$struct $(<$var_lifetime>)? > for $name<'a> {
                fn from(inner: $struct $(<$var_lifetime>)? ) -> Self {
                    Self::$variant(inner)
                }
            }
        )*

        // everything below is for internal usage within craftflow

        // the generated macro is used for the packet events
        gen_direction_enum!{__gen_macros $direction, enum $name { $( $variant($struct) ),* } }
	};
	(__gen_macros S2C, enum $name:ident { $( $variant:ident ( $struct:ident ) ),* } ) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __destructure_s2c__ {
            ($enum_value:ident -> $inner:ident $code:tt) => {
                match $enum_value {
                    $(
                        craftflow_protocol_abstract::$name::$variant($inner) => $code,
                    )*
                }
            };
        }

        #[doc(hidden)]
        #[macro_export]
        macro_rules! __gen_impls_for_packets_s2c {
            (impl $trait_name:ident for X $code:tt) => {
                $(
                    const _: () = {
                        type X = $crate::s2c::$struct;
                        impl $trait_name for X $code
                    };
                )*
            };
            (impl $trait_name:ident for Post<X> $code:tt) => {
                $(
                    const _: () = {
                        type X = $crate::s2c::$struct;
                        impl $trait_name for Post<X> $code
                    };
                )*
            };
        }
	};
	(__gen_macros C2S, enum $name:ident { $( $variant:ident ( $struct:ident ) ),* } ) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __destructure_c2s__ {
            ($enum_value:ident -> $inner:ident $code:tt) => {
                match $enum_value {
                    $(
                        craftflow_protocol_abstract::$name::$variant($inner) => $code,
                    )*
                }
            };
        }

        #[doc(hidden)]
        #[macro_export]
        macro_rules! __gen_impls_for_packets_c2s {
            (impl $trait_name:ident for X $code:tt) => {
                $(
                    const _: () = {
                        type X = $crate::c2s::$struct;
                        impl $trait_name for X $code
                    };
                )*
            };
        }
	};
}
