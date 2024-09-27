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
from gen_protocol import generate_protocol

def find_closest_below(defined_versions, version):
    versions = sorted(defined_versions.keys())
    closest_version = None

    for v in versions:
        if v < version:
            closest_version = v
        else:
            break

    return closest_version

def main():
    # first thing we gotta do is clone the minecraft-data repo
    # or fetch updates if already cloned
    repo_path = os.path.join(CACHE_DIR, "minecraft-data")
    if os.path.exists(repo_path):
        print(Fore.GREEN + Style.BRIGHT + "minecraft-data" + Fore.CYAN + " already cloned, fetching updates")

        os.system(f"cd {repo_path} && git fetch")
    else:
        print(Fore.CYAN + Style.BRIGHT + "Cloning " + Fore.GREEN + "minecraft-data")

        os.system(f"cd {CACHE_DIR} && git clone {REPOSITORY} minecraft-data")

    print(Fore.CYAN + Style.BRIGHT + "Checking out commit " + Fore.GREEN + COMMIT)

    os.system(f"cd {repo_path} && git checkout --force {COMMIT}")

    # first iterate over all defined versions in minecraft-data
    # to create a structure mapping protocol versions to their definition paths
    defined_versions = {}
    all_versions_dir = os.path.join(repo_path, "data", "pc")
    for version_dir in os.listdir(all_versions_dir):
        version_dir_path = os.path.join(all_versions_dir, version_dir)

        version_file = os.path.join(version_dir_path, "version.json")
        protocol_file = os.path.join(version_dir_path, "protocol.json")
        # only add if both version and protocol files exist
        if os.path.isfile(version_file) and os.path.isfile(protocol_file):
            with open(version_file, "r") as f:
                version_data = json.loads(f.read())

            defined_versions[version_data["version"]] = version_dir_path

    # Remove classic minecraft version which has no bussiness being here anyway and is
    # causing trouble with the conflicting protocol version
    # Hopefully later it will be deleted upstream and this can be removed
    for key, value in dict(defined_versions).items():
        if value == ".cache/minecraft-data/data/pc/0.30c":
            del defined_versions[key]

    # for debugging purposes
    for version in sorted(defined_versions.keys()):
        print(Fore.CYAN + "Found version " +
            Fore.YELLOW + Style.BRIGHT + str(version) +
            Fore.CYAN + Style.NORMAL + " at " +
            Fore.YELLOW + Style.BRIGHT + defined_versions[version])

    # now iterate over all versions and packets, finding any that are not already generated
    for version in range(VERSION_RANGE[0], VERSION_RANGE[1] + 1):
        # if this version is not defined that means its identical to its previous one
        if version not in defined_versions.keys():
            if find_closest_below(defined_versions, version) is None:
                print(Fore.RED + Style.BRIGHT + f"Version {version} not found")
                sys.exit(1)
            continue

        # find the most recent version that was defined before this version
        prev_version = find_closest_below(defined_versions, version)

        with open(os.path.join(defined_versions[version], "protocol.json"), "r") as f:
            protocol = json.loads(f.read())

        prev_protocol = None
        if prev_version is not None:
            with open(os.path.join(defined_versions[prev_version], "protocol.json"), "r") as f:
                prev_protocol = json.loads(f.read())

        if protocol == prev_protocol:
            # if identical to previous protocol, no need to generate anything
            # and remove from defined_versions so other versions dont try to re-export this
            del defined_versions[version]
            continue

        generate_protocol(version, protocol, prev_version, prev_protocol)

    print(Fore.GREEN + Style.BRIGHT + "Done")

if __name__ == '__main__':
    init(autoreset=True)
    main()
