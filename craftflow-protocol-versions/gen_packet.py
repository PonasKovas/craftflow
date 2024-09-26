import json
from openai import OpenAI

openai_client = OpenAI()

# Generates a rust implementation for a packet just from it's JSON specification using an LLM
def gen_packet(spec) -> str:
    with open('packet_prompt.txt', 'r') as file:
        prompt = file.read()

    compact_spec_json = json.dumps(spec, separators=(',', ':'))

    prompt = prompt.replace("{{packet_json}}", compact_spec_json)

    response = openai_client.chat.completions.create(
        messages=[
            {
                "role": "user",
                "content": prompt,
            }
        ],
        model="gpt-3.5-turbo",
        seed=0,
        temperature=0,
    ).choices[0].message.content

    return f"use craftflow_protocol_core::*;\nuse craftflow_protocol_core::datatypes::*;\n\n{response}"
