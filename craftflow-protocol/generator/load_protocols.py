from typing import Dict, Tuple
from colorama import Fore, Style
import json

from conf import *

# Given a dictionary of protocol versions mapped to their directories, loads all unique protocol.json and returns
# a dict mapping protocol versions to their protocols json


def load_protocols(versions: Dict[int, Path]) -> Tuple[Dict[int, any], Dict[int, int]]:
    protocols = {}
    aliases = {}
    for version, version_path in versions.items():
        with open(versions[version] / "protocol.json", "r") as f:
            p = json.loads(f.read())

        # just in case minecraft-data is even more retarded than i realize
        # if any other version protocol is identical to this one, we can just skip it
        identical = False
        for other_v, other in protocols.items():
            if other == p:
                identical = True
                break

        if identical:
            print(Fore.YELLOW + Style.BRIGHT + f"Skipping {version} due to being identical to {other_v}")
            aliases[version] = other_v
            continue

        protocols[version] = p

    return protocols, aliases
