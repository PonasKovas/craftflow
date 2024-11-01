from typing_extensions import Optional
import os
from colorama import Fore, Style

from conf import C2S_PACKETS, S2C_PACKETS
from gen_types import gen_types
from llm_gen import llm_gen

def snake_to_pascal(snake_str: str) -> str:
    return ''.join(word.capitalize() for word in snake_str.split('_'))

def get_packet_spec(protocol, direction: str, state: str, packet: str):
    if state not in protocol:
        return None

    packet = f"packet_{packet}"

    d = "toServer" if direction == "c2s" else "toClient"

    if packet not in protocol[state][d]["types"]:
        return None

    return protocol[state][d]["types"][packet]

def get_packet_id(protocol, direction: str, state: str, packet: str):
    d = "toServer" if direction == "c2s" else "toClient"

    mappings = protocol[state][d]["types"]["packet"][1][0]["type"][1]["mappings"]

    for id, name in mappings.items():
        if name == packet:
            return int(id, 16)

# Prepares the {direction}/{state}/{packet}/v{version}/ directory
# with all the mod.rs files for rust
# creating any if they dont already exist
def prepare_dir(direction, state, packet, version):
    # direction directory
    direction_path = f"{direction}/"
    if not os.path.exists(direction_path):
        os.makedirs(direction_path)

    # state directory
    state_path = os.path.join(direction_path, state)
    if not os.path.exists(state_path):
        os.makedirs(state_path)

    # packet directory
    packet_path = os.path.join(state_path, packet)
    if not os.path.exists(packet_path):
        os.makedirs(packet_path)

    # version directory
    version_path = os.path.join(packet_path, f"v{version:05}")
    if not os.path.exists(version_path):
        os.makedirs(version_path)

def generate_protocols_direction(all_protocols, direction: str):
    states = C2S_PACKETS if direction == "c2s" else S2C_PACKETS

    for state, packets in states.items():
        for packet in packets:
            # find all versions that have an identical packet
            # format:
            # [ [{packet_id: 0x00, version: 145}, {packet_id: 0x00, version: 156}] ]
            identical_versions = []
            # basically group identical versions together, while still allowing them to have different
            # packet ids.

            for v, p in all_protocols.items():
                spec = get_packet_spec(p, direction, state, packet)
                if spec is None:
                    continue

                packet_id = get_packet_id(p, direction, state, packet)

                # check if any version that we already iterated over has an identical packet
                found = False
                for v_list in identical_versions:
                    # each list must have at least one version
                    spec2 = get_packet_spec(all_protocols[v_list[0]["version"]], direction, state, packet)
                    if spec == spec2:
                        v_list.append({ "version": v, "packet_id": packet_id })
                        found = True
                        break

                if found:
                    continue

                # no identical packet found - add a new list
                identical_versions.append([{ "version": v, "packet_id": packet_id }])

            # now we can generate the groups of identical packets
            for group in identical_versions:
                for i, v in enumerate(group):
                    packet_id = v["packet_id"]
                    v = v["version"]

                    # check if already generated
                    if os.path.exists(f"{direction}/{state}/{packet}/v{v:05}/"):
                        continue

                    prepare_dir(direction, state, packet, v)

                    # if this is the first version in the group, generate the packet
                    # otherwise just re-export
                    if i == 0:
                        spec = get_packet_spec(all_protocols[v], direction, state, packet)

                        name = snake_to_pascal(packet) + f"V{v:05}"
                        print(f"Generating {direction} -> {state} -> {packet} -> {v:05} with an LLM")

                        # use the LLM to generate the packet definition
                        name, code = llm_gen(name, spec)

                        with open(f"{direction}/{state}/{packet}/v{v:05}/mod.rs", "w") as f:
                            f.write(code)

                        # add some info for the build.rs for generating enums
                        with open(f"{direction}/{state}/{packet}/v{v:05}/packet_id", "w") as f:
                            f.write(f"{packet_id}")
                            with open(f"{direction}/{state}/{packet}/v{v:05}/name", "w") as f:
                                f.write(f"{name}")
                    else:
                        # re-export the first version

                        with open(f"{direction}/{state}/{packet}/v{v:05}/mod.rs", "w") as f:
                            f.write(f"pub use crate::{direction}::{state}::{packet}::v{group[0]['version']:05}::*;\n")

                        # add some info for the build.rs for generating enums
                        with open(f"{direction}/{state}/{packet}/v{v:05}/packet_reexport", "w") as f:
                            f.write(f"{group[0]['version']}")
                        with open(f"{direction}/{state}/{packet}/v{v:05}/packet_id", "w") as f:
                            f.write(f"{packet_id}")


# all_protocols must be ordered by version ascending
def generate_protocols(all_protocols):
    generate_protocols_direction(all_protocols, "c2s")
    generate_protocols_direction(all_protocols, "s2c")
    gen_types(all_protocols)
