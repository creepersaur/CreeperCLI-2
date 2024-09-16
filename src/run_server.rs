use super::{server::run_server, settings, CWD, ROOT};
use colored::Colorize;
use std::io::{stdout, Write};
use std::mem::drop;
use std::{io::stdin, path::Path};

pub async fn start() {
    let mut port: u16 = 8080;

    if let Ok(settings) = settings::get_settings() {
        for (name, value) in settings.iter() {
            match name.to_lowercase().as_str() {
                "port" => port = value.as_integer().unwrap_or(8080) as u16,
                "root" => {
                    let mut data = match ROOT.lock() {
                        Ok(guard) => guard,
                        Err(poisoned) => poisoned.into_inner(),
                    };
                    *data = value.as_str().unwrap_or("game").to_string();
                    drop(data);
                }
                _ => {}
            }
        }
    }

    let root = match ROOT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let root_name = (*root).clone();
    drop(root);

    let game_dir = CWD.join(&root_name);
    let path = Path::new(&game_dir);

    if !(path.exists() && path.is_dir()) {
        println!(
            "{} {} {}",
            "YOU MUST HAVE A".red(),
            format!("`{}`", root_name).purple(),
            "DIRECTORY IN THE WORKING DIRECTORY.".red()
        );
        print!(
            "{}\n{}",
            "Try calling the `init` command to setup CreeperCLI.".yellow(),
            "Press [Enter] to close.".dimmed()
        );

        stdout().flush().expect("Failed to flush output.");
        stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line.");

        return;
    }

    println!(
        "{} {}",
        "Running server at:".bold().green(),
        format!("http://localhost:{}", port).purple()
    );
    run_server(port).await.expect("Failed to run server!");
}
