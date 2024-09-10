use colored::Colorize;
use lazy_static::lazy_static;
use std::{env, io::stdin, path::Path, sync::Mutex};
use filesystem::get_cwd;
use server::run_server;
use std::mem::drop;

mod sourcemap;
mod filesystem;
mod get;
mod post;
mod server;
mod settings;
mod update;

lazy_static! {
    // pub static ref DIRECTORIES: Mutex<Table> = Mutex::new(Table::new());
    pub static ref ROOT: Mutex<String> = Mutex::new("game".to_string());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cwd = get_cwd();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "update" {
        update::update_cli();
        return Ok(())
    }

    let mut port: u16 = 8080;

    if let Ok(settings) = settings::get_settings(&cwd) {
        for (name, value) in settings.iter() {
            match name.to_lowercase().as_str() {
                "port" => port = value.as_integer().unwrap_or(8080) as u16,
                "root" => {
                    let mut data = match ROOT.lock() {
                        Ok(guard) => guard,
                        Err(poisoned) => poisoned.into_inner(), // Recover from poisoned mutex
                    };
                    *data = value.as_str().unwrap_or("game").to_string();
                    drop(data);
                },
                _ => {}
            }
        }
    }
    
    let root = match ROOT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };
    let game_dir = format!("{}//{}", &cwd, *root);

    let path = Path::new(game_dir.as_str());
    if !(path.exists() && path.is_dir()) {
        println!(
            "{} {} {}",
            "YOU MUST HAVE A".red(),
            format!("`{}`",root).purple(),
            "DIRECTORY IN THE WORKING DIRECTORY.".red()
        );
        stdin()
            .read_line(&mut String::new())
            .expect("Failed to read_line.");
        return Ok(());
    }
    
    drop(root);

    println!(
        "{} {}",
        "Running server at:".bold().green(),
        format!("http://localhost:{}", port).purple()
    );
    Ok(run_server(port).await.expect("Failed to run server!"))
}
