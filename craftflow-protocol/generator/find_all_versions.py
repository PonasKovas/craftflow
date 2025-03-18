from typing import Dict
import os
import subprocess
from colorama import Fore, Style
import json

from conf import *

# Reads the minecraft-data repository and returns a dictionary mapping protocol
# versions to their protocol.json file paths


def find_all_versions() -> Dict[str, Path]:
    # first thing we gotta do is clone the minecraft-data repo
    # or fetch updates if already cloned
    repo_path = CACHE_DIR / "minecraft-data"

    if repo_path.exists():
        print(Fore.GREEN + Style.BRIGHT + "minecraft-data" +
              Fore.CYAN + " already cloned, fetching updates")
        subprocess.run("git fetch", shell=True, check=True, cwd=repo_path)
    else:
        print(Fore.CYAN + Style.BRIGHT + "Cloning " +
              Fore.GREEN + "minecraft-data")
        CACHE_DIR.mkdir(exist_ok=True)
        subprocess.run(f"git clone {REPOSITORY} minecraft-data", shell=True, check=True, cwd=CACHE_DIR)

    print(Fore.CYAN + Style.BRIGHT + "Checking out commit " + Fore.GREEN + COMMIT)
    subprocess.run(f"git checkout --force {COMMIT}", shell=True, check=True, cwd=repo_path)

    # read the data/pc/common/protocolVersions.json because it contains a list versions without the classic
    # versions that are present in this repository for whatever fucking reason
    with open(repo_path / "data" / "pc" / "common" / "protocolVersions.json", "r") as f:
        non_classic_versions = [item["minecraftVersion"]
                                for item in json.loads(f.read())]

    # iterate over all defined versions in minecraft-data
    # to create a structure mapping protocol versions to their definition paths
    versions = {}
    for version_dir in (repo_path / "data" / "pc").iterdir():
        # only add if both version.json and protocol.json files exist
        if not ((version_dir / "version.json").is_file() and (version_dir / "protocol.json").is_file()):
            print(Fore.YELLOW + Style.NORMAL + "Skipping " + Fore.MAGENTA +
                  str(version_dir) + Fore.YELLOW + " (no version.json or protocol.json)")
            continue

        with open(version_dir / "version.json", "r") as f:
            version_data = json.loads(f.read())

        # skip versions that are not in the common versions list (classic minecraft versions which we dont fucking need)
        if version_data["minecraftVersion"] not in non_classic_versions:
            print(Fore.YELLOW + Style.NORMAL + "Skipping " + Fore.MAGENTA +
                  str(version_dir) + Fore.YELLOW + " (not in common versions list)")
            continue

        # since minecraft-data is so fucking shitty and impossible to work with
        # (but im thankful for its existance 🙏🙏🙏 sorry)
        # we have to use this hack to check if the version is a release or a snapshot
        # basically if there are any chars except for numbers and ".", its not a release

        # there is also "releaseType" field in common protocol versions list, but only for SOME of the versions,
        # while the majority dont have it, making it pretty much useless but we check it too just in case
        if not all(char.isdigit() or char == '.' for char in version_data["minecraftVersion"]) or \
                ("releaseType" in version_data and version_data["releaseType"] != "release"):
            print(Fore.YELLOW + Style.NORMAL + "Skipping " + Fore.MAGENTA +
                  str(version_dir) + Fore.YELLOW + " (not release)")
            continue

        versions[version_data["version"]] = version_dir

    return versions
