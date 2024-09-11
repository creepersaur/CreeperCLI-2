use colored::Colorize;
use std::{io::stdin, path::Path};
use super::{
    server::run_server,
    CWD,
    settings,
    ROOT
};
use std::mem::drop;

pub async fn start() {
    let mut port: u16 = 8080;

    if let Ok(settings) = settings::get_settings() {
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
    let mut game_dir = CWD.clone();
    game_dir.push((*root).clone());

    let path = Path::new(&game_dir);
    if !(path.exists() && path.is_dir()) {
        println!(
            "{} {} {}",
            "YOU MUST HAVE A".red(),
            format!("`{}`",root).purple(),
            "DIRECTORY IN THE WORKING DIRECTORY.".red()
        );
        println!("{}", "Try calling the `init` command to setup CreeperCLI.");

        stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line.");
    }
    
    drop(root);

    println!(
        "{} {}",
        "Running server at:".bold().green(),
        format!("http://localhost:{}", port).purple()
    );
    run_server(port).await.expect("Failed to run server!");
}