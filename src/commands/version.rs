use colored::Colorize;

pub fn print_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("\n /‾-‾-‾-‾-‾-‾-‾-‾-‾-‾-‾-‾\\");
    println!("<   CreeperCLI : {}{}   >", "v".green(), version.bright_green().bold());
    println!(" \\_-_-_-_-_-_-_-_-_-_-_-_/");
}