import json
from openai import OpenAI
from pydantic import BaseModel
from conf import COMMIT

openai_client = OpenAI()

def llm_gen(name: str, spec) -> tuple[str, str]:
    name, code = llm_gen_inner(name, spec)

    spec_pretty = json.dumps(spec, indent=4)
    commented_spec = "\n".join("// " + line for line in spec_pretty.splitlines())

    note = f"// GENERATED // MINECRAFT-DATA COMMIT HASH {COMMIT} //"
    wall = "/" * len(note)

    return name, f"""{wall}
    {note}
    {wall}

    {commented_spec}


    {code}
    """

def llm_gen_inner(name, spec):
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
    # i would have loaded this as json but json doesnt support multiline strings
    # gotta interpret it as a python dict i guess
    prompt = eval(prompt)

    response = openai_client.chat.completions.create(
        messages=prompt,
        model="gpt-4o",
        seed=0,
        temperature=0,
        response_format={ "type": "json_object" },
    ).choices[0].message.content

    if response is None:
        return "Failed to generate packet"

    response = json.loads(response)

    return response["name"], response["rust_code"]
