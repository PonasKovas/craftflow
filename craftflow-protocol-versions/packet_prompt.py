[
    {
        "role": "system",
        "content": "You must respond with JSON in the format of {\"rust_code\": \"your code here\"}.",
    },
    {
        "role": "user",
        "content": """I have attached a JSON specification of a network protocol packet. I need you to write a structure and
                reading/writing implementation for it in the Rust language. Follow Rust best practices, the code must fit
                naturally in a Rust codebase. Change the structure of the packet to make invalid state impossible to construct.
                You do not need to rewrite the structure of the packet 1:1, change it to be simpler and more rust-like (most importantly
                make invalid state impossible by leveraging enums, BUT ONLY IF IT IS SPECIFIED, DO NOT ASSUME ANY INVALID STATE THAT IS
                NOT SPECIFIED), but keep the serialized binary format the same. You can define multiple structures or enums to represent
                deeper structure. Do not reply with any words or text formatting, ONLY the code. Do not use unwrap or expect.
                Do not make any assumptions about the packet that are not provided in the specification. If you make any unprovided
                assumptions or write anything more than is requested of you, you will be fired. All types that are used but not given a
                definition are ALREADY DEFINED and have MCPRead and MCPWrite traits implemented for them. Do not import anything,
                everything you need is already imported. Derive essential traits for all types (Debug, PartialEq, Clone, Hash, PartialOrd,
                Ord and Eq if possible).
                VarInt can be constructed with VarInt(n) where n is i32, and inner number accesed with varint.0
                Same for VarLong, but with i64.
                UUID is just u128.
                buffer is Buffer<LEN_TYPE>.
                array is Array<LEN_TYPE, ELEMENT_TYPE>.
                restBuffer is RestBuffer<ELEMENT_TYPE>.
                You will have to implement these types manually:
                - bitfield
                - topBitSetTerminatedArray (array where the first bit of an element signifies if another element follows)

                Here is the packet specification:
                Packet name: PacketUpdateAttributes
                JSON:
                ["container",[{"name":"entityId","type":"i32"},{"name":"properties","type":["array",{"countType":"i32","type":["container",[{"name":"key","type":"string"},{"name":"value","type":"f64"},{"name":"modifiers","type":["array",{"countType":"i16","type":["container",[{"name":"uuid","type":"UUID"},{"name":"amount","type":"f64"},{"name":"operation","type":"i8"}]]}]}]]}]}]]
        """,
    },
    {
        "role": "assistant",
        "content": """{"rust_code": "#[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
        pub struct PacketUpdateAttributes {
            pub entity_id: i32,
            pub properties: Vec<Property>,
        }
        #[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
        pub struct Property {
            pub key: String,
            pub value: f64
            pub modifiers: Vec<PropertyModifier>,
        }
        #[derive(Debug, PartialEq, Clone, Hash, PartialOrd)]
        pub struct PropertyModifier {
            pub uuid: u128,
            pub amount: f64
            pub operation: i8,
        }
        impl MCPWrite for PacketUpdateAttributes {
            fn write(&self, output: &mut impl std::io::Write) -> Result<usize> {
                let mut written_bytes = 0;

                written_bytes += self.entity_id.write(output)?;
                written_bytes += (self.properties.len() as i32).write(output)?;
                for property in &self.properties {
                    written_bytes += property.key.write(output)?;
                    written_bytes += property.value.write(output)?;
                    written_bytes += (property.modifiers.len() as i16).write(output)?;
                    for modifier in &property.modifiers {
                        written_bytes += modifier.uuid.write(output)?;
                        written_bytes += modifier.amount.write(output)?;
                        written_bytes += modifier.operation.write(output)?;
                    }
                }

                Ok(written_bytes)
            }
        }
        impl MCPRead for PacketUpdateAttributes {
            fn read(input: &[u8]) -> Result<(&[u8], Self)> {
                let (input, entity_id) = i32::read(input)?;
                let (mut input, properties_len) = i32::read(input)?;
                let mut properties = Vec::new();
                for _ in 0..properties_len {
                    let (i, key) = String::read(input)?;
                    let (i, value) = f64::read(input)?;
                    let (i, modifiers_len) = i16::read(input)?;
                    let mut modifiers = Vec::new();
                    for _ in 0..modifiers_len {
                        let (ii, uuid) = u128::read(input)?;
                        let (ii, amount) = f64::read(input)?;
                        let (ii, operation) = i8::read(input)?;
                        modifiers.push(PropertyModifier{ uuid, amount, operation });
                        i = ii;
                    }
                    properties.push(Property{ key, value, modifiers });
                    input = i;
                }

                Ok((input, Self { entity_id, properties }))
            }
        }
    "}""",
    },
    {
        "role": "user",
        "content": """Thank you. Good job, that was exactly what I needed. Now do the same with this:
        Packet name: {{{packet_name}}}
        {{{packet_json}}}""",
    }
]