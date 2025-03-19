from typing import Dict
from tomlkit import table, inline_table, dumps
import tomlkit
import subprocess
from pathlib import Path
from colorama import init, Fore, Style

from .parse_protocol import get_packet_id, get_packet_spec, has_packet

# will add entries to packets.toml and also generate any not-already generated packets using an LLM


def gen(args, toml, protocols: Dict[int, any], packets_impl_path: Path, direction: str, state: str, packet: str):
    # only load llm module if gen_llm flag passed
    # because otherwise OpenAI requires an API key
    if args.gen_llm:
        from .llm import llm_gen_packet_impl

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

            if spec == group_spec:
                # add it to the group
                if packet_id not in group:
                    group[packet_id] = []
                group[packet_id].append(v)
                found = True
                break

        if not found:
            # no identical packet found - add a new list
            identical_versions.append({packet_id: [v]})

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
            group_table.add(str(packet_id), versions)

        # actual rust code generation
        #############################

        # if implementation not generated yet - generate
        packet_impl_path = packets_impl_path / direction / state / packet
        impl_path = packet_impl_path / f"v{first_version}.rs"
        if not impl_path.exists():
            if not args.gen_llm:
                print(Fore.YELLOW + f"Not generating {impl_path} using an LLM. Use --gen_llm flag to generate")
                continue

            packet_impl_path.mkdir(parents=True, exist_ok=True)

            spec = get_packet_spec(
                protocols[first_version], direction, state, packet)
            code = llm_gen_packet_impl(packet, first_version, spec)

            with open(impl_path, "w") as f:
                f.write(code)
            subprocess.run(f"rustfmt --edition 2024 {impl_path}", shell=True, check=True)
