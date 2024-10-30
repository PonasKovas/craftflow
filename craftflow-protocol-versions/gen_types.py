import os

from conf import TYPES
from llm_gen import llm_gen

def snake_to_pascal(snake_str: str) -> str:
    return ''.join(word.capitalize() for word in snake_str.split('_'))

# Prepares the src/types/v{version}/{type}/ directory
# with all the mod.rs files for rust
# creating any if they dont already exist
def prepare_dir(type, version):
    # types directory
    types_path = f"src/types/"
    if not os.path.exists(types_path):
        os.makedirs(types_path)
        with open("src/lib.rs", "a") as f:
            f.write(f"pub mod types;\n")
        open(os.path.join(types_path, "mod.rs"), "w").close() # create empty mod.rs

    # version directory
    version_path = os.path.join(types_path, f"v{version:05}")
    if not os.path.exists(version_path):
        os.makedirs(version_path)
        with open(os.path.join(types_path, "mod.rs"), "a") as f:
            f.write(f"pub mod v{version:05};\n")
        open(os.path.join(version_path, "mod.rs"), "w").close() # create empty mod.rs

    # specific type directory
    type_path = os.path.join(version_path, type)
    if not os.path.exists(type_path):
        os.makedirs(type_path)
        with open(os.path.join(version_path, "mod.rs"), "a") as f:
            f.write(f"pub mod {type};\n")
            f.write(f"pub use {type}::{snake_to_pascal(type)};\n")



# Generates a rust implementation for a type just from it's JSON specification using an LLM
def gen_type(type, version, spec) -> str:
    name = snake_to_pascal(type)

    print(f"Generating type {type} -> {version:05} with an LLM")

    response = llm_gen(name, spec)

    return f"""
    #[allow(unused_imports)]
    use std::borrow::Cow;
    #[allow(unused_imports)]
    use craftflow_protocol_core::*;
    #[allow(unused_imports)]
    use craftflow_protocol_core::datatypes::*;

    {response}
    """


def gen_types(all_protocols):
    for type in TYPES:

        # find all versions that have an def of this type
        # format:
        # [ [int] version, [int] version ] ]
        identical_versions = []
        # basically group identical versions together

        for v, p in all_protocols.items():
            if type not in p["types"]:
                continue

            spec = p["types"][type]

            # check if any version that we already iterated over has an identical packet
            found = False
            for v_list in identical_versions:
                # each list must have at least one version
                spec2 = all_protocols[v_list[0]]["types"][type]
                if spec == spec2:
                    v_list.append(v)
                    found = True
                    break

            if found:
                continue

            # no identical packet found - add a new list
            identical_versions.append([v])

        # now we can generate the groups of identical types
        for group in identical_versions:
            for i, v in enumerate(group):
                # check if already generated
                if os.path.exists(f"src/types/v{v:05}/{type}/"):
                    continue

                prepare_dir(type, v)

                # if this is the first version in the group, generate the packet
                # otherwise just re-export
                if i == 0:
                    spec = all_protocols[v]["types"][type]

                    # use the LLM to generate the packet definition
                    code = gen_type(type, v, spec)

                    with open(f"src/types/v{v:05}/{type}/mod.rs", "w") as f:
                        f.write(code)
                else:
                    # re-export the first version

                    with open(f"src/types/v{v:05}/{type}/mod.rs", "w") as f:
                        f.write(f"pub use crate::types::v{group[0]:05}::{type}::*;\n")
