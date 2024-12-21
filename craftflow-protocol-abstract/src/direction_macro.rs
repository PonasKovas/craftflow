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
            type Constructor = Box<dyn crate::AbPacketConstructor<
                AbPacket = $name<'static>
            > + Send + Sync + 'static>;

            fn construct(
                packet: &'a Self::Direction,
            ) -> anyhow::Result<crate::ConstructorResult<Self, Self::Constructor>> {
                $(
                    match $struct::construct(packet)? {
                        crate::ConstructorResult::Ignore => {},
                        crate::ConstructorResult::Done(inner) => return Ok(crate::ConstructorResult::Done(Self::$variant(inner))),
                        crate::ConstructorResult::Continue(inner) => {
                            // A constructor wrapper that converts the result to the enum variant
                            struct __ConstructorWrapper(<$struct $(${ignore($var_lifetime)}<'static>)? as crate::AbPacketNew<'static>>::Constructor);
                            impl crate::AbPacketConstructor for __ConstructorWrapper {
                                type AbPacket = $name<'static>;

                                fn next_packet(
                                    &mut self,
                              		packet: crate::packet_constructor::ConcretePacket<'_>,
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

                            return Ok(crate::ConstructorResult::Continue(
                                Box::new(__ConstructorWrapper(inner))
                            ))},
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
        gen_direction_enum!{__gen_macros $direction, enum $name<'a> { $( $variant($struct  $( <$var_lifetime> )? ) ),* } }
	};
	(__gen_macros S2C, enum $name:ident<'a> { $( $variant:ident ( $struct:ident  $( <$var_lifetime:lifetime> )? ) ),* } ) => {
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
        macro_rules! __gen_events_for_packets_s2c {
            ($event_trait:ident, $eventargs_trait:ident, $pointer_trait:ident) => {
                $(
                    #[doc = concat!(
                        "Event for the [abstract S2C ",
                        stringify!($struct),
                        "][",
                        stringify!($crate),
                        "::s2c::",
                        stringify!($struct),
                        "] packet"
                    )]
    				pub struct ${concat(S2C, $struct, Event)};

     				impl<'a> $eventargs_trait<'a> for ${concat(S2C, $struct, Event)} {
    				    /// The connection ID and the packet
                        ///
                        /// Obviously, don't try to change the connection ID, as it will propagate to other handlers
    				    type Args = (u64, $crate::s2c::$struct $( <$var_lifetime> )?);
    				}
    				impl $event_trait for ${concat(S2C, $struct, Event)} {
                        type Return = ();
    				}

                    impl<'a> $pointer_trait<'a> for $crate::s2c::$struct $( <$var_lifetime> )? {
                        type Event = ${concat(S2C, $struct, Event)};
                    }
                )*
            };
        }
	};
	(__gen_macros C2S, enum $name:ident<'a> { $( $variant:ident ( $struct:ident  $( <$var_lifetime:lifetime> )? ) ),* } ) => {
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
        macro_rules! __gen_events_for_packets_c2s {
            ($event_trait:ident, $eventargs_trait:ident, $pointer_trait:ident) => {
                $(
                    #[doc = concat!(
                        "Event for the [abstract C2S ",
                        stringify!($struct),
                        "][",
                        stringify!($crate),
                        "::c2s::",
                        stringify!($struct),
                        "] packet"
                    )]
    				pub struct ${concat(C2S, $struct, Event)};

        			impl<'a> $eventargs_trait<'a> for ${concat(C2S, $struct, Event)} {
    				    /// The connection ID and the packet
    				    type Args = (u64, $crate::c2s::$struct $( <$var_lifetime> )?);
    				}
    				impl $event_trait for ${concat(C2S, $struct, Event)} {
                        type Return = ();
    				}

                    impl<'a> $pointer_trait<'a> for $crate::c2s::$struct $( <$var_lifetime> )? {
                        type Event = ${concat(C2S, $struct, Event)};
                    }
                )*
            };
        }
	};
}
