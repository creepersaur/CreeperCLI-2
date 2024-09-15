use colored::Colorize;
mod update;
mod init;
mod version;

pub fn run_command(args: Vec<String>) {
    match args[1].as_str() {
        "update" => update::update_cli().expect("Failed to update CreeperCLI."),
        "init" => init::initialize(args),
        "version" => version::print_version(),
        _ => println!("{} Could not find command `{}`.", "[NO_COMMAND]".red(), args[1])
    }
}