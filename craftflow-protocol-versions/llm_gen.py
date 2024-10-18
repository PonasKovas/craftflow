import json
from openai import OpenAI
from pydantic import BaseModel

openai_client = OpenAI()

def llm_gen(name, spec) -> str:
    with open('packet_prompt.py', 'r') as file:
        prompt = file.read()

    with open('prompt_example_spec.json', 'r') as file:
        example_spec = json.dumps(json.load(file), separators=(',', ':'))
    with open('prompt_example_code.rs', 'r') as file:
        example_code = file.read()

    compact_spec_json = json.dumps(spec, separators=(',', ':'))

    prompt = prompt.replace("{{{example_spec}}}", example_spec)
    prompt = prompt.replace("{{{example_code}}}", example_code)
    prompt = prompt.replace("{{{packet_json}}}", compact_spec_json)
    prompt = prompt.replace("{{{packet_name}}}", name)

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

    return response
