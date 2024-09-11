use lazy_static::lazy_static;
use colored::Colorize;
use std::{env, path::PathBuf, sync::Mutex};

mod filesystem;
mod get;
mod post;
mod server;
mod settings;
mod update;
mod run_server;
mod init;

lazy_static! {
    pub static ref ROOT: Mutex<String> = Mutex::new("game".to_string());
    pub static ref CWD: PathBuf = filesystem::get_cwd();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "update" => update::update_cli().expect("Failed to update CreeperCLI."),
            "init" => init::initialize(),
            _ => println!("{} Could not find command `{}`.", "[NO_COMMAND]".red(), args[1])
        }
    } else {
        run_server::start().await
    }

    Ok(())
}
