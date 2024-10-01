macro_rules! gen_direction_enum {
	(@DIRECTION=$direction:ident; $( #[$attr:meta] )* pub enum $name:ident { $( $variant:ident ( $struct:ident ) ),* $(,)? }) => {
	    $( #[$attr] )*
		pub enum $name {
		    $($variant($struct),)*
		}

		impl crate::AbPacketWrite for $name {
            type Direction = craftflow_protocol_versions::$direction;

            fn convert_and_write(
                self,
                protocol_version: u32,
                writer: impl FnMut(Self::Direction) -> craftflow_protocol_core::Result<()>,
            ) -> craftflow_protocol_core::Result<()> {
                match self {
                    $(
                        $name::$variant(inner) => inner.convert_and_write(protocol_version, writer),
                    )*
                }
            }
        }

        impl crate::AbPacketNew for $name {
            type Direction = craftflow_protocol_versions::$direction;
            type Constructor = Box<dyn crate::AbPacketConstructor<Direction = Self::Direction, AbPacket = Self>>;

            fn construct(
                mut packet: Self::Direction,
            ) -> craftflow_protocol_core::Result<crate::ConstructorResult<Self, Self::Constructor, Self::Direction>> {


                $(
                    packet = match $struct::construct(packet)? {
                        crate::ConstructorResult::Done(inner) => return Ok(crate::ConstructorResult::Done(Self::$variant(inner))),
                        crate::ConstructorResult::Continue(inner) => {
                            // A constructor wrapper that converts the result to the enum variant
                            struct __ConstructorWrapper(<$struct as crate::AbPacketNew>::Constructor);
                            impl crate::AbPacketConstructor for __ConstructorWrapper {
                                type Direction = craftflow_protocol_versions::$direction;
                                type AbPacket = $name;

                                fn next_packet(
                              		self,
                              		packet: Self::Direction,
                               	) -> craftflow_protocol_core::Result<
                                    crate::ConstructorResult<Self::AbPacket, Self, (Self, Self::Direction)>
                                > {
                                    match self.0.next_packet(packet)? {
                                        crate::ConstructorResult::Done(inner) =>
                                            Ok(crate::ConstructorResult::Done($name::$variant(inner))),
                                        crate::ConstructorResult::Continue(inner) =>
                                            Ok(crate::ConstructorResult::Continue(Self(inner))),
                                        crate::ConstructorResult::Ignore((inner, packet)) =>
                                            Ok(crate::ConstructorResult::Ignore((Self(inner), packet))),
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

        // the generated macro is used internally in craftflow for the packet events
        gen_direction_enum!{__gen_destructure_macro $direction, pub enum $name { $( $variant($struct) ),* } }
	};
	(__gen_destructure_macro S2C, pub enum $name:ident { $( $variant:ident ( $struct:ident ) ),* } ) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __destructure_s2c__ {
            ($enum_value:ident -> $code:tt) => {
                match $enum_value {
                    $(
                        $name::$variant(inner) => $code,
                    )*
                }
            };
        }
	};
	(__gen_destructure_macro C2S, pub enum $name:ident { $( $variant:ident ( $struct:ident ) ),* } ) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __destructure_c2s__ {
            ($enum_value:ident -> $code:tt) => {
                match $enum_value {
                    $(
                        $name::$variant(inner) => $code,
                    )*
                }
            };
        }
	};
}
