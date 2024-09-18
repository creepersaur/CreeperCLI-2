use lazy_static::lazy_static;
use std::{env, path::PathBuf, sync::Mutex};

mod filesystem;
mod get;
mod post;
mod server;
mod settings;
mod commands;
mod run_server;

lazy_static! {
    pub static ref ROOT: Mutex<String> = Mutex::new("game".to_string());
    pub static ref CWD: PathBuf = filesystem::get_cwd();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        commands::run_command(args).await;
    } else {
        run_server::start().await;
    }

    Ok(())
}
