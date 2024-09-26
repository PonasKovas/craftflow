import json
from openai import OpenAI
from pydantic import BaseModel

openai_client = OpenAI()

# Generates a rust implementation for a packet just from it's JSON specification using an LLM
def gen_packet(packet_name, spec) -> str:
    with open('packet_prompt.txt', 'r') as file:
        prompt = file.read()

    compact_spec_json = json.dumps(spec, separators=(',', ':'))

    prompt = prompt.replace("{{packet_json}}", compact_spec_json)
    prompt = prompt.replace("{{packet_name}}", packet_name)

    response = openai_client.chat.completions.create(
        messages=[
            {
                "role": "system",
                "content": "You must respond with JSON in the format of {\"rust_code\": \"your code here\"}.",
            },
            {
                "role": "user",
                "content": prompt,
            }
        ],
        model="gpt-4o-mini",
        seed=0,
        temperature=0,
        response_format={ "type": "json_object" },
    ).choices[0].message.content

    if response is None:
        return "Failed to generate packet"

    response = json.loads(response)["rust_code"]

    return f"use craftflow_protocol_core::*;\nuse craftflow_protocol_core::datatypes::*;\n\n{response}"
