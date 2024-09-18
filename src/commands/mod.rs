use actix_web::rt::task::spawn_blocking;
use colored::Colorize;
mod init;
mod pizza;
mod update;
mod version;

pub async fn run_command(args: Vec<String>) {
    match args[1].as_str() {
        "update" => {
            spawn_blocking(move || update::update_cli().expect("Failed to update CreeperCLI."));
        }
        "init" => init::initialize(args),
        "version" => version::print_version(),
        "pizza" => pizza::spawn_pizza(),
        _ => println!(
            "{} Could not find command `{}`.",
            "[NO_COMMAND]".red(),
            args[1]
        ),
    }
}
