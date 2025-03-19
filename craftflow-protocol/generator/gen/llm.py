import json
from openai import OpenAI

openai_client = OpenAI()
MODEL = "gpt-4o"


def llm_gen_packet_impl(packet: str, version: int, spec) -> str:
    with open('prompt.py', 'r') as file:
        prompt = file.read()

    example_name = "EntityInformationV18"
    with open('example_spec.json', 'r') as file:
        example_spec = json.dumps(json.load(file), separators=(',', ':'))
    with open('example_code.rs', 'r') as file:
        example_code = file.read()

    packet_name = f"{snake_to_pascal_case(packet)}V{version}"
    packet_spec = json.dumps(spec, separators=(',', ':'))

    prompt = prompt.replace("{{{example_name}}}", example_name)
    prompt = prompt.replace("{{{example_spec}}}", example_spec)
    prompt = prompt.replace("{{{example_code}}}", example_code)
    prompt = prompt.replace("{{{packet_name}}}", packet_name)
    prompt = prompt.replace("{{{packet_spec}}}", packet_spec)

    # LOL. this just makes everything easier
    prompt = eval(prompt)

    response = openai_client.chat.completions.create(
        messages=prompt,
        model=MODEL,
        seed=0,
        temperature=0,
        response_format={"type": "json_object"},
    ).choices[0].message.content

    code = json.loads(response)["rust_code"]

    spec_pretty = json.dumps(spec, indent=4)
    commented_spec = "\n".join(
        "// " + line for line in spec_pretty.splitlines())

    return name, f"""{commented_spec}


    {code}
    """


def snake_to_pascal_case(snake_str):
    # Split the string by underscores, capitalize each part, and join them back together
    return ''.join(word.capitalize() for word in snake_str.split('_'))
