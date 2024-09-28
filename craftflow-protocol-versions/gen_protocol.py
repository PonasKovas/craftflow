from typing_extensions import Optional
import os
from colorama import Fore, Style

from conf import C2S_PACKETS, S2C_PACKETS
from gen_packet import gen_packet

def snake_to_pascal(snake_str):
    return ''.join(word.capitalize() for word in snake_str.split('_'))

def get_packet_spec(protocol, state: str, packet: str, c2s: bool):
    if state not in protocol:
        return None

    d = "toServer" if c2s else "toClient"

    if packet not in protocol[state][d]["types"]:
        return None

    return protocol[state][d]["types"][packet]

# Prepares the src/v{version}/{dir_mod_name}/{state}/ directory
# with all the mod.rs files for rust
def prepare_dir(version, dir_mod_name, state):
    # create version directory
    path = f"src/v{version:05}/"
    if not os.path.exists(path):
        os.makedirs(path)
        with open("src/lib.rs", "a") as f:
            f.write(f"pub mod v{version:05};\n")
        open(os.path.join(path, "mod.rs"), "w").close()

    # create direction directory
    path2 = os.path.join(path, dir_mod_name)
    if not os.path.exists(path2):
        os.makedirs(path2)
        with open(os.path.join(path, "mod.rs"), "a") as f:
            f.write(f"pub mod {dir_mod_name};\n")
            f.write(f"include!(concat!(env!(\"OUT_DIR\"), \"/v{version:05}/{dir_mod_name}.rs\"));\n\n")
        open(os.path.join(path2, "mod.rs"), "w").close()

    path3 = os.path.join(path2, state)
    if not os.path.exists(path3):
        os.makedirs(path3)
        with open(os.path.join(path2, "mod.rs"), "a") as f:
            f.write(f"pub mod {state};\n")
            f.write(f"include!(concat!(env!(\"OUT_DIR\"), \"/v{version:05}/{dir_mod_name}/{state}.rs\"));\n\n")
        open(os.path.join(path3, "mod.rs"), "w").close()

def generate_protocol_direction(version: int, protocol, prev_version: Optional[int], prev_protocol, c2s: bool):
    packets = C2S_PACKETS if c2s else S2C_PACKETS

    dir_mod_name = "c2s" if c2s else "s2c"

    for packet in packets:
        state, packet = packet.split("/")

        # check if already generated
        if os.path.exists(f"src/v{version:05}/{dir_mod_name}/{state}/{packet}.rs"):
            continue

        spec = get_packet_spec(protocol, state, packet, c2s)

        if spec is None:
            continue

        prev_spec = get_packet_spec(prev_protocol, state, packet, c2s) if prev_protocol else None

        if spec == prev_spec:
            # just re-export from previous version
            generated_rust = f"pub use crate::v{prev_version:05}::{dir_mod_name}::{state}::{packet}::{snake_to_pascal(packet)};"
        else:
            print(Fore.GREEN + Style.BRIGHT + f"Generating {packet} for {state} in {dir_mod_name} in {version}")
            generated_rust = gen_packet(snake_to_pascal(packet), spec)

        prepare_dir(version, dir_mod_name, state)

        with open(f"src/v{version:05}/{dir_mod_name}/{state}/mod.rs", "a") as f:
            f.write(f"pub mod {packet};\n")

        with open(f"src/v{version:05}/{dir_mod_name}/{state}/{packet}.rs", "w") as f:
            f.write(generated_rust)

def generate_protocol(version: int, protocol, prev_version: Optional[int], prev_protocol):
    generate_protocol_direction(version, protocol, prev_version, prev_protocol, True)
    generate_protocol_direction(version, protocol, prev_version, prev_protocol, False)
