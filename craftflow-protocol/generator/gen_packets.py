from typing import Dict, List
from tomlkit import table, inline_table, dumps
import tomlkit
import subprocess
from pathlib import Path
from colorama import init, Fore, Style

from conf import *
from parse_protocol import get_packet_id, get_packet_spec, has_packet, get_type_spec


def add_aliased_versions(version_aliases: Dict[int, int], versions: List[int]) -> List[int]:
    for alias, v in version_aliases.items():
        if v in versions:
            versions.append(alias)

    return versions


# Checks if the given spec contains a keyvalue pair "type": "<type>" anywhere recursively
def does_use_type(spec, type: str) -> bool:
    if isinstance(spec, dict):
        for key, value in spec.items():
            if key == "type" and value == type:
                return True
            elif does_use_type(value, type):
                return True
    elif isinstance(spec, list):
        for item in spec:
            if does_use_type(item, type):
                return True

    return False


# compares a packet spec in two different versions, including its used types
def compare_spec(protocols: Dict[int, any], v1: int, v2: int, spec1, spec2) -> bool:
    if spec1 != spec2:
        return False

    # check for types
    for type in TYPES:
        type = type.split(".")
        typename = type[-1]
        if not does_use_type(spec1, typename):
            continue

        # the type is used in the spec. gotta make sure that its the same for both versions
        type_spec1 = get_type_spec(protocols, v1, type)
        type_spec2 = get_type_spec(protocols, v2, type)
        if not compare_spec(protocols, v1, v2, type_spec1, type_spec2):
            return False

    # everything matches
    return True


# will add entries to packets.toml and also generate any not-already generated packets using an LLM
def gen_packets(toml, version_aliases: Dict[int, int], protocols: Dict[int, any], direction: str, state: str, packet: str):
    # only load llm module if gen-llm flag passed
    # because otherwise OpenAI requires an API key
    if ARGS.gen_llm:
        from llm import llm_gen_impl

    # find all versions that have an identical packet
    # format:
    # [
    #    # pkt_id: [versions]
    #    { 0x00: [145, 156, ...], 0x01: [159, 161, ...], ... },
    # ]
    identical_versions = []
    for v, p in protocols.items():
        if not has_packet(p, direction, state, packet):
            continue

        spec = get_packet_spec(p, direction, state, packet)
        packet_id = get_packet_id(p, direction, state, packet)

        # check if any version that we already iterated over has an identical packet
        found = False
        for group in identical_versions:
            # each list must have at least one version
            first_version = group[next(iter(group))][0]
            group_spec = get_packet_spec(
                protocols[first_version], direction, state, packet)

            if compare_spec(protocols, v, first_version, spec, group_spec):
                # add it to the group
                if packet_id not in group:
                    group[packet_id] = []
                group[packet_id].append(v)
                found = True
                break

        if not found:
            # no identical packet found - add a new list
            identical_versions.append({packet_id: [v]})

    if len(identical_versions) == 0:
        print(Fore.RED + f"NOT FOUND {direction} -> {state} -> {packet}")

    # now we can generate the groups of identical packets
    for group in identical_versions:
        first_version = group[next(iter(group))][0]

        # packets.toml generation
        #########################

        group_table = table()
        toml.add(str(first_version), group_table)
        group_table.add(tomlkit.comment(
            "<packet id> = [<versions that use that packet id>]"))
        for packet_id, versions in group.items():
            all_versions = add_aliased_versions(version_aliases, versions)
            group_table.add(str(packet_id), all_versions)

        # actual rust code generation
        #############################

        # if implementation not generated yet - generate
        packet_impl_path = PACKETS_IMPL_PATH / direction / state / packet
        impl_path = packet_impl_path / f"v{first_version}.rs"
        if not impl_path.exists():
            if not ARGS.gen_llm:
                print(Fore.YELLOW + f"Not generating {impl_path} using an LLM. Use --gen-llm flag to generate")
                continue

            packet_impl_path.mkdir(parents=True, exist_ok=True)

            spec = get_packet_spec(
                protocols[first_version], direction, state, packet)
            code = llm_gen_impl(packet, first_version, spec)

            with open(impl_path, "w") as f:
                f.write(code)
            subprocess.run(f"rustfmt --edition 2024 {impl_path}", shell=True, check=True)

            print(Fore.MAGENTA + f"Generated {impl_path} using an LLM.")
