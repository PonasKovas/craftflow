#!/usr/bin/env python

from colorama import init, Fore, Style
from conf import C2S_PACKETS, S2C_PACKETS, CACHE_DIR, VERSION_RANGE
import os
import json

def check_missing_packets():
    # Load protocol data like in main.py
    repo_path = os.path.join(CACHE_DIR, "minecraft-data")
    if not os.path.exists(repo_path):
        print(Fore.RED + "minecraft-data repository not found")
        return

    # Load protocol versions from common data
    with open(os.path.join(repo_path, "data", "pc", "common", "protocolVersions.json"), "r") as f:
        common_protocol_versions = json.loads(f.read())


    defined_versions = {}
    all_versions_dir = os.path.join(repo_path, "data", "pc")
    for version_dir in os.listdir(all_versions_dir):
        version_dir_path = os.path.join(all_versions_dir, version_dir)
        version_file = os.path.join(version_dir_path, "version.json")
        protocol_file = os.path.join(version_dir_path, "protocol.json")

        if not (os.path.isfile(version_file) and os.path.isfile(protocol_file)):
            continue

        with open(version_file, "r") as f:
            version_data = json.loads(f.read())

        # Skip versions not in common versions list
        skip = True
        for v in common_protocol_versions:
            if v["minecraftVersion"] == version_data["minecraftVersion"]:
                skip = False
                break
        if skip:
            continue

        # Skip snapshots
        if not all(char.isdigit() or char == '.' for char in version_data["minecraftVersion"]):
            continue

        with open(protocol_file, "r") as f:
            protocol = json.loads(f.read())

        defined_versions[version_data["version"]] = protocol

    defined_packets = {}
    # "packet_name" -> { direction: .., state: .., version: .. }

    # Collect all unique packets from all protocol versions
    for version in range(VERSION_RANGE[0], VERSION_RANGE[1] + 1):
        if version not in defined_versions:
            continue

        for state, inner in defined_versions[version].items():
            if state == "types":
                continue

            for direction, inner in inner.items():
                for packet, inner in inner["types"].items():
                    if packet == "packet":
                        continue

                    packet = packet.removeprefix("packet_")

                    if direction == "toClient" and state in S2C_PACKETS and packet in S2C_PACKETS[state] or \
                        direction == "toServer" and state in C2S_PACKETS and packet in C2S_PACKETS[state]:
                        continue

                    if packet not in defined_packets:
                        defined_packets[packet] = { "direction": direction, "state": state, "version": version }

    for packet, info in defined_packets.items():
        print(Fore.CYAN + Style.BRIGHT + "Missing packet " + Fore.YELLOW + packet + Fore.CYAN + \
            " direction=" + Fore.MAGENTA + info['direction'] + Fore.CYAN + " state=" + Fore.MAGENTA + \
            info['state'] + Fore.CYAN + " version=" + Fore.MAGENTA + str(info['version']))

if __name__ == '__main__':
    init(autoreset=True, strip=False)
    check_missing_packets()
