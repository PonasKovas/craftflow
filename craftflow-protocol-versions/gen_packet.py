import json
from openai import OpenAI
from pydantic import BaseModel

openai_client = OpenAI()

def snake_to_pascal(snake_str: str) -> str:
    return ''.join(word.capitalize() for word in snake_str.split('_'))


# Generates a rust implementation for a packet just from it's JSON specification using an LLM
def gen_packet(spec, direction: str, state: str, packet: str, version: int) -> str:
    with open('packet_prompt.py', 'r') as file:
        prompt = file.read()

    with open('prompt_example_spec.json', 'r') as file:
        example_spec = json.dumps(json.load(file), separators=(',', ':'))
    with open('prompt_example_code.rs', 'r') as file:
        example_code = file.read()

    compact_spec_json = json.dumps(spec, separators=(',', ':'))

    packet_name = snake_to_pascal(packet)
    struct_name = packet_name + f"V{version:05}"

    prompt = prompt.replace("{{{example_spec}}}", example_spec)
    prompt = prompt.replace("{{{example_code}}}", example_code)
    prompt = prompt.replace("{{{packet_json}}}", compact_spec_json)
    prompt = prompt.replace("{{{packet_name}}}", struct_name)

    # LOL.
    # i would have loaded this as json but gotta interpret it as a python dict
    # so I can use multiline strings
    prompt = eval(prompt)

    response = openai_client.chat.completions.create(
        messages=prompt,
        model="gpt-4o-mini",
        seed=0,
        temperature=0,
        response_format={ "type": "json_object" },
    ).choices[0].message.content

    if response is None:
        return "Failed to generate packet"

    response = json.loads(response)["rust_code"]

    print(f"Generated {direction} -> {state} -> {packet} -> {version:05} with an LLM")

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
