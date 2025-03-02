CACHE_DIR = ".cache/"
REPOSITORY = "https://github.com/PrismarineJS/minecraft-data.git"
COMMIT = "f1130aea931b948d2ecaecf34ecfe0116bfd4172"
VERSION_RANGE = [5, 767]
C2S_PACKETS = {
    "handshaking": ["set_protocol"],
    "status": ["ping_start", "ping"],
    "login": ["login_start", "encryption_begin", "login_plugin_response", "login_acknowledged"],
    "configuration": ["settings", "custom_payload", "finish_configuration", "keep_alive", "pong",
        "resource_pack_receive"],
}
S2C_PACKETS = {
    "status": ["server_info", "ping"],
    "login": ["disconnect", "encryption_begin", "success", "compress", "login_plugin_request"],
    "configuration": ["custom_payload", "disconnect", "finish_configuration", "keep_alive", "ping",
        "registry_data", "remove_resource_pack", "add_resource_pack", "feature_flags", "tags", "reset_chat"],
}
TYPES = ["tags"]
