from typing import Dict
from tomlkit import table, inline_table, dumps
import tomlkit
import subprocess
from pathlib import Path
from colorama import init, Fore, Style

from conf import *

# will add entries to packets.toml and also generate any not-already generated packets using an LLM


def gen_types(args, toml, protocols: Dict[int, any], ty: str):
    # only load llm module if gen_llm flag passed
    # because otherwise OpenAI requires an API key
    if args.gen_llm:
        from llm import llm_gen_impl

    # find all versions that have an def of this type
    # format:
    # [ [int] ] - a list of lists of versions that have identical type def
    identical_versions = []
    # basically group identical versions together
    for v, p in protocols.items():
        if ty not in p["types"]:
            continue

        spec = p["types"][ty]

        # check if any version that we already iterated over has an identical packet
        found = False
        for v_list in identical_versions:
            # each list must have at least one version
            spec2 = protocols[v_list[0]]["types"][ty]
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
        first_version = group[0]

        # packets.toml generation
        #########################

        toml.add(str(first_version), group)

        # actual rust code generation
        #############################

        # if implementation not generated yet - generate
        type_impl_path = TYPES_IMPL_PATH / ty
        impl_path = type_impl_path / f"v{first_version}.rs"
        if not impl_path.exists():
            if not args.gen_llm:
                print(Fore.YELLOW + f"Not generating {impl_path} using an LLM. Use --gen_llm flag to generate")
                continue

            type_impl_path.mkdir(parents=True, exist_ok=True)

            spec = protocols[first_version]["types"][ty]
            code = llm_gen_impl(ty, first_version, spec)

            with open(impl_path, "w") as f:
                f.write(code)
            subprocess.run(f"rustfmt --edition 2024 {impl_path}", shell=True, check=True)

            print(Fore.MAGENTA + f"Generated {impl_path} using an LLM.")
