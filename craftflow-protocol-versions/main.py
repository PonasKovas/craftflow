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

    # read the data/pc/common/protocolVersions.json because it contains all versions without the classic
    # versions that are present in this repository for whatever fucking reason
    with open(os.path.join(repo_path, "data", "pc", "common", "protocolVersions.json"), "r") as f:
        common_protocol_versions = json.loads(f.read())

    # iterate over all defined versions in minecraft-data
    # to create a structure mapping protocol versions to their definition paths
    defined_versions = {}
    all_versions_dir = os.path.join(repo_path, "data", "pc")
    for version_dir in os.listdir(all_versions_dir):
        version_dir_path = os.path.join(all_versions_dir, version_dir)

        version_file = os.path.join(version_dir_path, "version.json")
        protocol_file = os.path.join(version_dir_path, "protocol.json")
        # only add if both version and protocol files exist
        if not (os.path.isfile(version_file) and os.path.isfile(protocol_file)):
            print(Fore.YELLOW + Style.BRIGHT + f"Skipping version {version_dir} (no version.json or protocol.json)")
            continue

        with open(version_file, "r") as f:
            version_data = json.loads(f.read())

        # skip versions that are not in the common versions list (this will be the classic minecraft versions)
        skip = True
        for v in common_protocol_versions:
            if v["minecraftVersion"] == version_data["minecraftVersion"]:
                skip = False
                break

        if skip:
            print(Fore.YELLOW + Style.BRIGHT + f"Skipping version {version_dir} (not in common versions list)")
            continue

        # since minecraft-data is so fucking shitty and impossible to work with
        # we have to use this hack to check if the version is a release or a snapshot
        if not all(char.isdigit() or char == '.' for char in version_data["minecraftVersion"]):
            # there is "releaseType" field in common protocol versions list, but only for SOME of the versions,
            # while the majority dont have it, making it completely useless
            print(Fore.YELLOW + Style.BRIGHT + f"Skipping version {version_dir} (not release)")
            continue

        defined_versions[version_data["version"]] = version_dir_path

    # for debugging purposes
    for version in sorted(defined_versions.keys()):
        print(Fore.CYAN + "Found version " +
            Fore.YELLOW + Style.BRIGHT + str(version) +
            Fore.CYAN + Style.NORMAL + " at " +
            Fore.YELLOW + Style.BRIGHT + defined_versions[version])

    # load all the protocol.json files into a dictionary
    protocols = {}
    for version in range(VERSION_RANGE[0], VERSION_RANGE[1] + 1):
        prev_version = find_closest_below(list(protocols.keys()), version)
        # if this version is not defined that means its identical to its previous one
        # and there is nothing to generate
        if version not in defined_versions.keys():
            if prev_version is None:
                print(Fore.RED + Style.BRIGHT + f"First version {version} not found")
                sys.exit(1)
            continue

        with open(os.path.join(defined_versions[version], "protocol.json"), "r") as f:
            protocol = json.loads(f.read())

        # just in case minecraft-data is even more retarded than i realize
        # if the previous version protocol is identical to this one, we can just skip it
        if prev_version is not None and protocols[prev_version] == protocol:
            print(Fore.YELLOW + Style.BRIGHT + f"Skipping {version} due to being identical to {prev_version}")
            continue

        protocols[version] = protocol

    generate_protocols(protocols)

    # also set the supported version list for rust code
    with open("src/supported_versions.rs", "w") as f:
        f.write("pub const MIN_VERSION: u32 = " + str(VERSION_RANGE[0]) + ";\n")
        f.write("pub const MAX_VERSION: u32 = " + str(VERSION_RANGE[1]) + ";\n")

    print(Fore.GREEN + Style.BRIGHT + "Formatting")
    os.system("cargo fmt")

    print(Fore.GREEN + Style.BRIGHT + "Done")

if __name__ == '__main__':
    init(autoreset=True)
    main()
