def has_packet(protocol, direction: str, state: str, packet: str) -> bool:
    if state not in protocol:
        return False

    direction = "toServer" if direction == "c2s" else "toClient"

    if packet not in protocol[state][direction]["types"]["packet"][1][1]["type"][1]["fields"]:
        return False

    return True


def get_packet_type_name(protocol, direction: str, state: str, packet: str):
    direction = "toServer" if direction == "c2s" else "toClient"

    return protocol[state][direction]["types"]["packet"][1][1]["type"][1]["fields"][packet]


def get_packet_spec(protocol, direction: str, state: str, packet: str):
    packet_type_name = get_packet_type_name(protocol, direction, state, packet)

    direction = "toServer" if direction == "c2s" else "toClient"

    if packet_type_name in protocol[state][direction]["types"]:
        return protocol[state][direction]["types"][packet_type_name]

    if packet_type_name in protocol["types"]:
        return protocol["types"][packet_type_name]

    return None


def get_packet_id(protocol, direction: str, state: str, packet: str) -> int:
    direction = "toServer" if direction == "c2s" else "toClient"

    mappings = protocol[state][direction]["types"]["packet"][1][0]["type"][1]["mappings"]

    for id, name in mappings.items():
        if name == packet:
            return int(id, 16)
