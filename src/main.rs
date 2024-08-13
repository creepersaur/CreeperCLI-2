use colored::Colorize;
use toml::Table;
use std::{fs, io::stdin, path::Path};

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

    let mut port: u16 = 8080;

    if let Ok(creeper_toml) = fs::read_to_string(format!("{}//creeper.toml", &cwd)) {
        if let Ok(settings) = creeper_toml.parse::<Table>() {
            for (name, value) in settings.iter() {
                match name.to_lowercase().as_str() {
                    "port" => port = value.as_integer().unwrap_or(8080) as u16,
                    _ => {}
                }
            }
        }
    }

    let path = Path::new(game_dir.as_str());
    if !(path.exists() && path.is_dir()) {
        println!(
            "{}",
            "YOU MUST HAVE A `game` DIRECTORY IN THE WORKING DIRECTORY.".red()
        );
        stdin()
            .read_line(&mut String::new())
            .expect("Failed to read_line.");
        return Ok(());
    }

    println!(
        "{} {}",
        "Running server at:".bold().green(),
        format!("http://localhost:{}", port).purple()
    );
    Ok(run_server(port).await.expect("Failed to run server!"))
}
