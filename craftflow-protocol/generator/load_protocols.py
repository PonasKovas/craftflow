from typing import Dict
from colorama import Fore, Style
import json

from conf import *

# Given a dictionary of protocol versions mapped to their directories, loads all unique protocol.json and returns
# a dict mapping protocol versions to their protocols json
def load_protocols(versions: Dict[int, Path]) -> Dict[int, any]:
	protocols = {}
	prev_version = None
	for version, version_path in versions.items():
		with open(versions[version] / "protocol.json", "r") as f:
			p = json.loads(f.read())

		# just in case minecraft-data is even more retarded than i realize
		# if the previous version protocol is identical to this one, we can just skip it
		if prev_version is not None and protocols[prev_version] == p:
			print(Fore.YELLOW + Style.BRIGHT + f"Skipping {version} due to being identical to {prev_version}")
			continue
		prev_version = version

		protocols[version] = p

	return protocols