#!/bin/env python

# This uses OpenAI API to generate rust code for the packet specifications from PrismarineJS/minecraft-data
#
# Running this script will generate any packets or versions that are not found already generated in the project

from colorama import init, Fore, Style
from conf import *
from find_all_versions import find_all_versions
from load_protocols import load_protocols
from gen_packets import gen_packets
from gen_types import gen_types
from tomlkit import table, inline_table, dumps, document
import tomlkit


def pascal_to_snake_case(s: str) -> str:
    if not s:
        return ""

    result = s[0].lower()
    for char in s[1:]:
        if char.isupper():
            result += "_"
        result += char.lower()
    return result


def main():
    versions = find_all_versions()

    # for debugging purposes
    for version in sorted(versions.keys()):
        print(Fore.CYAN + "Found version " +
              Fore.YELLOW + Style.BRIGHT + str(version) +
              Fore.CYAN + Style.NORMAL + " at " +
              Fore.YELLOW + Style.BRIGHT + str(versions[version]))

    # remove all versions that we are not interested in
    for version in list(versions.keys()):
        if version not in range(VERSION_RANGE[0], VERSION_RANGE[1] + 1):
            del versions[version]

    # sort ascending by protocol version
    versions = dict(sorted(versions.items()))

    # load all the protocol.json files into a dictionary
    protocols, version_aliases = load_protocols(versions)

    # initialise the packets.toml file
    packets_toml = document()
    packets_toml.add(tomlkit.comment(
        "AUTOMATICALLY GENERATED FROM PrismarineJS/minecraft-data"))
    packets_toml.add(tomlkit.comment(
        "NOT TO BE EDITED MANUALLY. SEE generator/ INSTEAD"))
    packets_toml.add(tomlkit.nl())

    # add the list of supported versions
    packets_toml.add("versions", list(versions.keys()))

    version_aliases_table = table()
    packets_toml.add("version_aliases", version_aliases_table)
    version_aliases_table.add(tomlkit.comment(
        "some versions are identical to the previous one protocol-wise"))
    for v, alias in version_aliases.items():
        version_aliases_table.add(str(v), alias)

    for direction, states in PACKETS.items():
        direction_table = table(True)
        packets_toml.add(direction, direction_table)
        for state, packets in states.items():
            state_table = table(True)
            direction_table.add(state, state_table)
            for packet in packets:
                packet_table = table(True)
                state_table.add(packet, packet_table)
                gen_packets(packet_table, version_aliases, protocols,
                            direction, state, packet)

    # Generate types
    types_table = table(True)
    packets_toml.add("type", types_table)
    for ty in TYPES:
        type_segments = ty.split(".")
        for i in range(len(type_segments)):
            type_segments[i] = pascal_to_snake_case(type_segments[i])

        parent = types_table
        for type_seg in type_segments:
            type_table = table()
            parent.add(type_seg, type_table)
            parent = type_table

        type_table.add(tomlkit.comment("<group id> = [<versions>]"))
        gen_types(type_table, version_aliases, protocols, type_segments)

    # write the packets.toml
    with open(PACKETS_TOML_PATH, "w") as f:
        f.write(dumps(packets_toml))

    # also add a feature for each supported version to the Cargo.toml
    with open(CARGO_TOML_PATH, "r") as f:
        cargo_toml_lines = f.readlines()

    marker_start_pos = None
    marker_end_pos = None
    for i, line in enumerate(cargo_toml_lines):
        if CARGO_TOML_START_MARKER in line and marker_start_pos is None:
            marker_start_pos = i
        elif CARGO_TOML_END_MARKER in line and marker_end_pos is None:
            marker_end_pos = i

    if marker_start_pos is None or marker_end_pos is None:
        print(Fore.RED + Style.BRIGHT +
              "Cargo.toml markers for autogenerating features not found!")
    else:
        cargo_toml_lines[marker_start_pos+1:marker_end_pos] = [f"no-v{v} = []\n" for v in versions.keys()]

        # Write back the modified TOML
        with open(CARGO_TOML_PATH, "w") as f:
            f.writelines(cargo_toml_lines)

        print(Fore.MAGENTA + "Cargo.toml features generated!")

    print(Fore.GREEN + Style.BRIGHT + "Done")


if __name__ == "__main__":
    init(autoreset=True)
    main()
