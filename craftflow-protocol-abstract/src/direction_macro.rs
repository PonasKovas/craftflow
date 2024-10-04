macro_rules! gen_direction_enum {
	(@DIRECTION=$direction:ident; $( #[$attr:meta] )* pub enum $name:ident { $( $variant:ident ( $struct:ident ) ),* $(,)? }) => {
	    $( #[$attr] )*
		pub enum $name {
		    $($variant($struct),)*
		}

		impl crate::AbPacketWrite for $name {
            type Direction = craftflow_protocol_versions::$direction;
            type Iter = Box<dyn Iterator<Item = Self::Direction> + Send + Sync>;

            fn convert(
          		self,
          		protocol_version: u32,
           	) -> anyhow::Result<Self::Iter> {
                Ok(match self {
                    $(
                        $name::$variant(inner) => Box::new(inner.convert(protocol_version)?),
                    )*
                })
            }
        }

        impl crate::AbPacketNew for $name {
            type Direction = craftflow_protocol_versions::$direction;
            type Constructor = Box<dyn crate::AbPacketConstructor<Direction = Self::Direction, AbPacket = Self> + Send + Sync>;

            fn construct(
                mut packet: Self::Direction,
            ) -> anyhow::Result<crate::ConstructorResult<Self, Self::Constructor, Self::Direction>> {


                $(
                    packet = match $struct::construct(packet)? {
                        crate::ConstructorResult::Done(inner) => return Ok(crate::ConstructorResult::Done(Self::$variant(inner))),
                        crate::ConstructorResult::Continue(inner) => {
                            // A constructor wrapper that converts the result to the enum variant
                            #[repr(transparent)]
                            struct __ConstructorWrapper(<$struct as crate::AbPacketNew>::Constructor);
                            impl crate::AbPacketConstructor for __ConstructorWrapper {
                                type Direction = craftflow_protocol_versions::$direction;
                                type AbPacket = $name;

                                fn next_packet(
                              		self: Box<Self>,
                              		packet: Self::Direction,
                               	) -> anyhow::Result<
                                    crate::ConstructorResult<
                                        Self::AbPacket,
                                        Box<dyn crate::AbPacketConstructor<
                                            AbPacket = Self::AbPacket,
                                            Direction = Self::Direction
                                        > + Send + Sync>,
                                        (Box<dyn crate::AbPacketConstructor<
                                            AbPacket = Self::AbPacket,
                                            Direction = Self::Direction
                                        > + Send + Sync>, Self::Direction)>
                                > {
                                    // its safe because the newtype is repr(transparent)
                                    // this shenanigan is needed because we can't return Self if it's not Sized
                                    // which forces us to use Box<Self> in the signature
                                    let inner = unsafe {
                                        Box::from_raw(Box::into_raw(self) as *mut <$struct as crate::AbPacketNew>::Constructor)
                                    };
                                    match crate::AbPacketConstructor::next_packet(inner, packet)? {
                                        crate::ConstructorResult::Done(inner) =>
                                            Ok(crate::ConstructorResult::Done($name::$variant(inner))),
                                        crate::ConstructorResult::Continue(inner) =>
                                            Ok(crate::ConstructorResult::Continue(unsafe {
                                                Box::from_raw(Box::into_raw(inner) as *mut Self)
                                            })),
                                        crate::ConstructorResult::Ignore((inner, packet)) =>
                                            Ok(crate::ConstructorResult::Ignore((unsafe {
                                                Box::from_raw(Box::into_raw(inner) as *mut Self)
                                            }, packet))),
                                    }
                                }
                            }

                            return Ok(crate::ConstructorResult::Continue(Box::new(__ConstructorWrapper(inner))))},
                        crate::ConstructorResult::Ignore(p) => p,
                    };
                )*

                Ok(crate::ConstructorResult::Ignore(packet))
            }
        }

        $(
            impl From<$struct> for $name {
                fn from(inner: $struct) -> Self {
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
                    impl $trait_name for $crate::s2c::$struct $code
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
                    impl $trait_name for $crate::c2s::$struct $code
                )*
            };
        }
	};
}
