use colored::Colorize;

use super::filesystem as fs;

pub fn initialize(args: Vec<String>) {
    let mut root = "game";
    if args.len() > 2 {
        if args[2] == "src" {
            root = "src";
        }
    }

    fs::create_file(
        &"creeper.toml",
        &format!(r#"
# Your root/src directory.
root = "{root}"
# The port to host the sever at.
port = 8080
# Enable two_way_sync on the following (array)
two_way_sync = [
    "ServerStorage",
    "ServerScriptService"
]
# Should include descendants when syncing back?
two_way_descendants = true
"#,)
    );

    fs::build_dir(format!("{root}/ServerScriptService/server"));
    fs::build_dir(format!("{root}/StarterPlayerScripts/client"));
    fs::build_dir(format!("{root}/ReplicatedStorage/shared"));

    fs::create_file(
        &format!("{root}/ServerScriptService/server/hello.server.luau"),
        &r#"print("Hello from CreeperCLI! (server)")"#
    );

    fs::create_file(
        &"sourcemap.json",
        &r#"["Will be replaced when the plugin connects."]"#
    );

    fs::create_file(
        &"default.project.json",
        &r#"["Will be replaced when the plugin connects."]"#
    );
    
    println!("{} 👍", "Successfully initialized CreeperCLI project!".green());
}
