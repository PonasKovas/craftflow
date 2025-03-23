from pathlib import Path

PACKETS_TOML_PATH = Path("../packets.toml")
CARGO_TOML_PATH = Path("../Cargo.toml")
CARGO_TOML_START_MARKER = "# START AUTO FEATURES SECTOR #"
CARGO_TOML_END_MARKER = "# END AUTO FEATURES SECTOR #"
PACKETS_IMPL_PATH = Path("../packets/")
CACHE_DIR = Path(".cache/")
REPOSITORY = "https://github.com/PrismarineJS/minecraft-data.git"
COMMIT = "89afb7586417a3b3a64d3ffca26dc96dddb7ae50"
VERSION_RANGE = [5, 769]
PACKETS = {
    "c2s": {
        "handshaking": ["set_protocol"],
        "status": ["ping_start", "ping"],
        "login": ["login_start", "encryption_begin", "login_plugin_response", "login_acknowledged"],
        "configuration": ["settings", "custom_payload", "finish_configuration", "keep_alive", "pong",
                          "resource_pack_receive"],
    },
    "s2c": {
        "status": ["server_info", "ping"],
        "login": ["disconnect", "encryption_begin", "success", "compress", "login_plugin_request"],
        "configuration": ["custom_payload", "disconnect", "finish_configuration", "keep_alive", "ping",
                          "registry_data", "remove_resource_pack", "add_resource_pack", "feature_flags", "tags", "reset_chat"],
    }
}
