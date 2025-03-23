def has_packet(protocol, direction: str, state: str, packet: str) -> bool:
    if state not in protocol:
        return False

    packet = f"packet_{packet}"
    direction = "toServer" if direction == "c2s" else "toClient"

    if packet not in protocol[state][direction]["types"]:
        return False

    return True


def get_packet_spec(protocol, direction: str, state: str, packet: str):
    direction = "toServer" if direction == "c2s" else "toClient"

    return protocol[state][direction]["types"][f"packet_{packet}"]


def get_packet_id(protocol, direction: str, state: str, packet: str) -> int:
    direction = "toServer" if direction == "c2s" else "toClient"

    mappings = protocol[state][direction]["types"]["packet"][1][0]["type"][1]["mappings"]

    for id, name in mappings.items():
        if name == packet:
            return int(id, 16)
