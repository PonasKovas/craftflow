#!/bin/env python

# This uses OpenAI API to generate rust code for the packet specifications from PrismarineJS/minecraft-data
#
# Running this script will generate any packets or versions that are not found already generated in the project

from typing_extensions import Optional
import os
import sys
import json
from colorama import init, Fore, Style

from conf import CACHE_DIR, REPOSITORY, COMMIT, VERSION_RANGE
from gen_protocol import generate_protocols
from get_defined_versions import get_defined_versions

# finds the closest version that is below the given version
def find_closest_below(versions, version):
    closest_version = None

    for v in versions:
        if v < version:
            closest_version = v
        else:
            break

    return closest_version

def main():
    defined_versions = get_defined_versions()

    # for debugging purposes
    for version in sorted(defined_versions.keys()):
        print(Fore.CYAN + "Found version " +
            Fore.YELLOW + Style.BRIGHT + str(version) +
            Fore.CYAN + Style.NORMAL + " at " +
            Fore.YELLOW + Style.BRIGHT + defined_versions[version])

    # load all the protocol.json files into a dictionary
    protocols = {}
    for version in range(VERSION_RANGE[0], VERSION_RANGE[1] + 1):
        # if this version is not defined that means its identical to its previous one
        # and we dont need to load anything
        if version not in defined_versions.keys():
            continue

        with open(os.path.join(defined_versions[version], "protocol.json"), "r") as f:
            protocol = json.loads(f.read())

        # just in case minecraft-data is even more retarded than i realize
        # if the previous version protocol is identical to this one, we can just skip it
        prev_version = find_closest_below(list(protocols.keys()), version)
        if prev_version is not None and protocols[prev_version] == protocol:
            print(Fore.YELLOW + Style.BRIGHT + f"Skipping {version} due to being identical to {prev_version}")
            continue

        protocols[version] = protocol

    generate_protocols(protocols)

    # also set the supported version list for rust code
    with open("src/supported_versions.rs", "w") as f:
        f.write("// THIS FILE IS AUTOMATICALLY GENERATED BY main.py //\n")
        f.write("/////////////////////////////////////////////////////\n")
        f.write("pub const MIN_VERSION: u32 = " + str(VERSION_RANGE[0]) + ";\n")
        f.write("pub const MAX_VERSION: u32 = " + str(VERSION_RANGE[1]) + ";\n")

    print(Fore.GREEN + Style.BRIGHT + "Done")

if __name__ == '__main__':
    init(autoreset=True)
    main()
