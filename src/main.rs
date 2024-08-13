use std::path::Path;
use colored::Colorize;

use filesystem::get_cwd;
use server::run_server;

mod filesystem;
mod get;
mod post;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cwd = get_cwd();
    let game_dir = format!("{}//game", &cwd);

    let path = Path::new(game_dir.as_str());
    if !(path.exists() && path.is_dir()) {
        println!(
            "{}",
            "YOU MUST HAVE A `game` DIRECTORY IN THE WORKING DIRECTORY.".red()
        );
        return Ok(());
    }

    println!("{} {}", "Running server at:".bold().green(), "http://localhost:8080".purple());
    Ok(run_server().await.expect("Failed to run server!"))
}
