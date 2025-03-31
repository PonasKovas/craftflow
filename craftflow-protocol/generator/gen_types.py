from typing import Dict, List
from tomlkit import table, inline_table, dumps
import tomlkit
import subprocess
from pathlib import Path
from colorama import init, Fore, Style

from parse_protocol import get_type_spec
from conf import *


def add_aliased_versions(version_aliases: Dict[int, int], versions: List[int]) -> List[int]:
    for alias, v in version_aliases.items():
        if v in versions:
            versions.append(alias)

    return versions


# will add entries to packets.toml and also generate any not-already generated packets using an LLM
def gen_types(toml, version_aliases: Dict[int, int], protocols: Dict[int, any], ty: List[str]):
    # only load llm module if gen-llm flag passed
    # because otherwise OpenAI requires an API key
    if ARGS.gen_llm:
        from llm import llm_gen_impl

    # find all versions that have a def of this type
    # format:
    # [ [int] ] - a list of lists of versions that have identical type def
    identical_versions = []
    # basically group identical versions together
    for v in protocols.keys():
        spec = get_type_spec(protocols, v, ty)

        if spec is None:
            continue

        # check if any version that we already iterated over has an identical packet
        found = False
        for v_list in identical_versions:
            # each list must have at least one version
            spec2 = get_type_spec(protocols, v_list[0], ty)
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

        all_versions = add_aliased_versions(version_aliases, group)
        toml.add(str(first_version), all_versions)

        # actual rust code generation
        #############################

        # if implementation not generated yet - generate
        type_impl_path = TYPES_IMPL_PATH
        for type_segment in ty:
            type_impl_path /= type_segment

        impl_path = type_impl_path / f"v{first_version}.rs"
        if not impl_path.exists():
            if not ARGS.gen_llm:
                print(Fore.YELLOW + f"Not generating {impl_path} using an LLM. Use --gen-llm flag to generate")
                continue

            type_impl_path.mkdir(parents=True, exist_ok=True)

            spec = get_type_spec(protocols, first_version, ty)
            code = llm_gen_impl(ty[-1], first_version, spec)

            with open(impl_path, "w") as f:
                f.write(code)
            subprocess.run(f"rustfmt --edition 2024 {impl_path}", shell=True, check=True)

            print(Fore.MAGENTA + f"Generated {impl_path} using an LLM.")
