from typing_extensions import Optional
import os
from colorama import Fore, Style

from conf import C2S_PACKETS, S2C_PACKETS
from gen_packet import gen_packet

def snake_to_pascal(snake_str):
    return ''.join(word.capitalize() for word in snake_str.split('_'))

def get_packet_spec(protocol, state: str, packet: str, direction: str):
    if state not in protocol:
        return None

    packet = f"packet_{packet}"

    d = "toServer" if direction == "c2s" else "toClient"

    if packet not in protocol[state][d]["types"]:
        return None

    return protocol[state][d]["types"][packet]

# Prepares the src/v{version}/{dir_mod_name}/{state}/ directory
# with all the mod.rs files for rust
# creating any if they dont already exist
def prepare_dir(version, direction, state):
    # create version directory
    path = f"src/v{version:05}/"
    if not os.path.exists(path):
        os.makedirs(path)
        with open("src/lib.rs", "a") as f:
            f.write(f"pub mod v{version:05};\n")
        open(os.path.join(path, "mod.rs"), "w").close()

    # create direction directory
    path2 = os.path.join(path, direction)
    if not os.path.exists(path2):
        os.makedirs(path2)
        with open(os.path.join(path, "mod.rs"), "a") as f:
            f.write(f"pub mod {direction};\n")
            f.write(f"include!(concat!(env!(\"OUT_DIR\"), \"/v{version:05}/{direction}.rs\"));\n\n")
        open(os.path.join(path2, "mod.rs"), "w").close()

    path3 = os.path.join(path2, state)
    if not os.path.exists(path3):
        os.makedirs(path3)
        with open(os.path.join(path2, "mod.rs"), "a") as f:
            f.write(f"pub mod {state};\n")
            f.write(f"include!(concat!(env!(\"OUT_DIR\"), \"/v{version:05}/{direction}/{state}.rs\"));\n\n")
        open(os.path.join(path3, "mod.rs"), "w").close()

def generate_protocol_direction(version: int, protocol, all_protocols, direction: str):
    states = C2S_PACKETS if direction == "c2s" else S2C_PACKETS

    for state, packets in states.items():
        for packet in packets:
            # check if already generated
            if os.path.exists(f"src/v{version:05}/{direction}/{state}/{packet}.rs"):
                continue

            spec = get_packet_spec(protocol, state, packet, direction)

            if spec is None:
                continue

            # check if any other version has an identical packet
            # saving the first one found
            first_identical = None
            identical_versions = []
            for v, p in all_protocols.items():
                prev_spec = get_packet_spec(p, state, packet, direction)
                if spec == prev_spec:
                    if first_identical is None:
                        first_identical = v
                    identical_versions.append(v)

            if first_identical != version:
                # there is a identical packet already defined - reuse it
                first_identical_path = f"crate::v{first_identical:05}::{direction}::{state}::{packet}::{snake_to_pascal(packet)}"
                generated_rust = f"""
                pub struct {snake_to_pascal(packet)}(pub {first_identical_path});

                impl crate::EqvPacket<{first_identical_path}> for {snake_to_pascal(packet)} {{
                    fn into_eqv_packet(self) -> {first_identical_path} {{
                        self.0
                    }}
                   	fn from_eqv_packet(p: {first_identical_path}) -> Self {{
                        Self(p)
                    }}
                }}

                impl std::ops::Deref for {snake_to_pascal(packet)} {{
                    type Target = {first_identical_path};

                    fn deref(&self) -> &Self::Target {{
                        &self.0
                    }}
                }}
                """
            else:
                # generate a new definition for this new packet
                print(Fore.GREEN + Style.BRIGHT +
                    f"Generating {packet} for {state} in {direction} in {version} (for versions {identical_versions})"
                )
                generated_rust = gen_packet(snake_to_pascal(packet), spec)

                generated_rust += f"""
                impl crate::EqvPacket<{snake_to_pascal(packet)}> for {snake_to_pascal(packet)} {{
                    fn into_eqv_packet(self) -> {snake_to_pascal(packet)} {{
                        self
                    }}
                   	fn from_eqv_packet(p: {snake_to_pascal(packet)}) -> Self {{
                        p
                    }}
                }}
                """

            generated_rust += f"""
            impl crate::Packet for {snake_to_pascal(packet)} {{
               	type Direction = crate::{direction.upper()};
               	type Version = crate::v{version:05}::{direction.upper()};
               	type State = crate::v{version:05}::{direction}::{snake_to_pascal(state)};

               	fn into_state_enum(self) -> Self::State {{
              		crate::v{version:05}::{direction}::{snake_to_pascal(state)}::{snake_to_pascal(packet)}(self)
               	}}
               	fn into_version_enum(self) -> Self::Version {{
              		crate::v{version:05}::{direction.upper()}::{snake_to_pascal(state)}(self.into_state_enum())
               	}}
               	fn into_direction_enum(self) -> Self::Direction {{
              		crate::{direction.upper()}::V{version:05}(self.into_version_enum())
               	}}
            }}

            impl crate::PacketVersion for {snake_to_pascal(packet)} {{
                const VERSIONS: &'static [u32] = &{identical_versions};
            }}
            """

            prepare_dir(version, direction, state)
            with open(f"src/v{version:05}/{direction}/{state}/mod.rs", "a") as f:
                f.write(f"pub mod {packet};\n")

            with open(f"src/v{version:05}/{direction}/{state}/{packet}.rs", "w") as f:
                f.write(generated_rust)

# all_protocols must be ordered by version ascending
def generate_protocol(version: int, protocol, all_protocols):
    generate_protocol_direction(version, protocol, all_protocols, "c2s")
    generate_protocol_direction(version, protocol, all_protocols, "s2c")
