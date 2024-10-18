from llm_gen import llm_gen

def snake_to_pascal(snake_str: str) -> str:
    return ''.join(word.capitalize() for word in snake_str.split('_'))

# Generates a rust implementation for a packet just from it's JSON specification using an LLM
def gen_packet(spec, direction: str, state: str, packet: str, version: int) -> str:
    packet_name = snake_to_pascal(packet)
    struct_name = packet_name + f"V{version:05}"

    print(f"Generating {direction} -> {state} -> {packet} -> {version:05} with an LLM")

    response = llm_gen(struct_name, spec)

    return f"""
    #[allow(unused_imports)]
    use craftflow_protocol_core::*;
    #[allow(unused_imports)]
    use craftflow_protocol_core::datatypes::*;

    {response}

    impl crate::IntoVersionEnum for {struct_name} {{
        type Packet = super::super::{packet_name};

    	fn into_version_enum(self) -> Self::Packet {{
            super::super::{packet_name}::V{version:05}(self)
        }}
    }}
    impl crate::IntoPacketEnum for {struct_name} {{
        type State = super::super::super::{snake_to_pascal(state)};

    	fn into_packet_enum(self) -> Self::State {{
            let packet = crate::IntoVersionEnum::into_version_enum(self);
            super::super::super::{snake_to_pascal(state)}::{packet_name}(packet)
        }}
    }}
    impl crate::IntoStateEnum for {struct_name} {{
        type Direction = super::super::super::super::{direction.upper()};

    	fn into_state_enum(self) -> Self::Direction {{
            let state = crate::IntoPacketEnum::into_packet_enum(self);
            super::super::super::super::{direction.upper()}::{snake_to_pascal(state)}(state)
        }}
    }}
    """
