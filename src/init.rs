use super::filesystem as fs;

pub fn initialize() {
    fs::create_file(
        &"creeper.toml",
        &r#"
# Your root/src directory.
root = "game"
# The port to host the sever at.
port = 8080
# Enable two_way_sync on the following (array)
two_way_sync = [
    "ServerStorage",
    "ServerScriptService"
]
# Should include descendants when syncing back?
two_way_descendants = true
"#,
    );

    fs::build_dir("game/ServerScriptService/server");
    fs::build_dir("game/StarterPlayerScripts/client");
    fs::build_dir("game/ReplicatedStorage/shared");

    fs::create_file(
        &"game/ServerScriptService/server/hello.server.luau",
        &r#"print("Hello from CreeperCLI! (server)")"#
    );
}
