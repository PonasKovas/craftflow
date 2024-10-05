CACHE_DIR = ".cache/"
REPOSITORY = "https://github.com/PrismarineJS/minecraft-data.git"
COMMIT = "9c8c31f2cee73500130e14e398a4b6ac6d5f22b8"
VERSION_RANGE = [5, 765]
C2S_PACKETS = {
    "handshaking": ["set_protocol"],
    "status": ["ping_start", "ping"],
}
S2C_PACKETS = {
    "status": ["server_info", "ping"],
    "login": ["disconnect", "encryption_begin", "success", "compress", "login_plugin_request"],
}
