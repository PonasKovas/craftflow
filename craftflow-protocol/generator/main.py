#!/bin/env python

# This uses OpenAI API to generate rust code for the packet specifications from PrismarineJS/minecraft-data
#
# Running this script will generate any packets or versions that are not found already generated in the project

from colorama import init, Fore, Style

from conf import *
from find_all_versions import find_all_versions
from load_protocols import load_protocols
from gen import gen
from tomlkit import table, inline_table, dumps, document
import tomlkit


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
    protocols = load_protocols(versions)

    # initialise the packets.toml file
    packets_toml = document()
    packets_toml.add(tomlkit.comment(
        "AUTOMATICALLY GENERATED FROM PrismarineJS/minecraft-data"))
    packets_toml.add(tomlkit.comment(
        "NOT TO BE EDITED MANUALLY. SEE generator/ INSTEAD"))
    packets_toml.add(tomlkit.nl())

    # add the list of supported versions
    packets_toml.add("versions", list(versions.keys()))

    for direction, states in PACKETS.items():
        direction_table = table(True)
        packets_toml.add(direction, direction_table)
        for state, packets in states.items():
            state_table = table(True)
            direction_table.add(state, state_table)
            for packet in packets:
                packet_table = table(True)
                state_table.add(packet, packet_table)
                gen(packet_table, protocols, PACKETS_IMPL_PATH,
                    direction, state, packet)

    # write the packets.toml
    with open(PACKETS_TOML_PATH, "w") as f:
        f.write(dumps(packets_toml))

    print(Fore.GREEN + Style.BRIGHT + "Done")


if __name__ == "__main__":
    init(autoreset=True)
    main()
