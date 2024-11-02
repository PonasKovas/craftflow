import os
from colorama import init, Fore, Style
import json

from conf import CACHE_DIR, REPOSITORY, COMMIT

# Reads the minecraft-data repository and returns a dictionary mapping protocol versions
# to their protocol.json file paths
def get_defined_versions():
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

        # skip versions that are not in the common versions list (classic minecraft versions which we dont fucking need)
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
        # basically if there are any chars except for numbers and ".", its not a release
        if not all(char.isdigit() or char == '.' for char in version_data["minecraftVersion"]):
            # there is "releaseType" field in common protocol versions list, but only for SOME of the versions,
            # while the majority dont have it, making it completely useless
            print(Fore.YELLOW + Style.BRIGHT + f"Skipping version {version_dir} (not release)")
            continue

        defined_versions[version_data["version"]] = version_dir_path

    return defined_versions
