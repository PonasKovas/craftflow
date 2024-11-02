#!/usr/bin/env python

# Running this script just prints a list of all packets that are defined in minecraft-data
# but not added to conf.py lists to be implemented.

from colorama import init, Fore, Style
from conf import C2S_PACKETS, S2C_PACKETS, CACHE_DIR, VERSION_RANGE
import os
import json

from get_defined_versions import get_defined_versions

def check_missing_packets():
    defined_versions = get_defined_versions()

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
