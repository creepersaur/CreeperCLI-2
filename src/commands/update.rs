use std::fs;
use std::io::Cursor;
use std::process::{exit, Command};
use colored::Colorize;

#[cfg(target_os = "windows")]
static OS: &str = "windows";
#[cfg(target_os = "linux")]
static OS: &str = "linux";
#[cfg(target_os = "macos")]
static OS: &str = "macos";

#[cfg(target_os = "windows")]
const FILE_NAME: &str = "creeper_cli-x86_64-pc-windows-msvc.zip";
#[cfg(target_os = "linux")]
const FILE_NAME: &str = "creeper_cli-x86_64-unknown-linux-gnu.tar.gz";
#[cfg(target_os = "macos")]
const FILE_NAME: &str = "creeper_cli-x86_64-apple-darwin.tar.gz";

pub fn update_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to update!");

    let latest_release_url =
        "https://api.github.com/repos/creepersaur/creepercli-2/releases/latest";
    let client = reqwest::blocking::Client::new();

    print!("{}", "Getting latest version assets... ".dimmed());

    let response = client
        .get(latest_release_url)
        .header(reqwest::header::USER_AGENT, "creepercli-updater")
        .send()?
        .json::<serde_json::Value>()?;

    let assets = response["assets"].as_array().ok_or("No assets found")?;
    let download_url = assets
        .iter()
        .find(|asset| {
            asset["name"]
                .as_str()
                .map_or(false, |name| name.contains(FILE_NAME))
        })
        .and_then(|asset| asset["browser_download_url"].as_str())
        .ok_or("Asset not found")?;

    println!("{}", "Latest release assets found.".dimmed());
    print!("{}", "Downloading assets... ".dimmed());

    let response = client
        .get(download_url)
        .header(reqwest::header::USER_AGENT, "creepercli-updater")
        .send()?;
    let bytes = response.bytes()?;

    println!("{}", "Downloaded zip file.".dimmed());
    print!("{}", "Extracting compressed file... ".dimmed());

    if OS == "windows" {
        unzip_and_replace(&bytes)?;
    } else {
        untar_and_replace(&bytes)?;
    }

    Ok(())
}

fn unzip_and_replace(zip_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let reader = Cursor::new(zip_data);
    let mut zip = zip::ZipArchive::new(reader)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name().ends_with(".exe") {
            let mut out_file = fs::File::create("creeper_cli_new.exe")?;
            std::io::copy(&mut file, &mut out_file)?;
            break;
        }
    }

    println!("{}", "Extracted file successfully.".dimmed());
    println!("{}", "Replacing binary...".dimmed());

    // Create a batch script to replace the running executable
    let current_exe = std::env::current_exe()?;
    let current_exe_str = current_exe.to_str().unwrap();

    let batch_script_content = format!(
        r#"
        @echo off
        color 0A
        timeout /t 2 /nobreak >nul
        move /Y creeper_cli_new.exe "{current_exe_str}"
    echo Successfully updated CreeperCLI-2! You may close this window.
    
    color 07
    exit
        "#,
        current_exe_str = current_exe_str
    );

    let temp_dir = std::env::temp_dir();
    let batch_file_path = temp_dir.join("update.bat");
    let batch_file_path_str = batch_file_path.to_str().unwrap();
    fs::write(batch_file_path_str, batch_script_content)?;

    println!("{}", "Successfully updated CreeperCLI-2! 👍".bright_green());

    // Run the batch script
    Command::new("cmd")
        .args(&["/C", "start", batch_file_path_str])
        .spawn()?;

    // Exit the current process so the batch can complete the replacement
    exit(0);
}

fn untar_and_replace(tar_gz_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let tar = GzDecoder::new(Cursor::new(tar_gz_data));
    let mut archive = Archive::new(tar);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        if path.file_name().map_or(false, |name| name == "creeper_cli") {
            entry.unpack("creeper_cli")?;
            break;
        }
    }

    println!("{}", "Extracted file successfully.".dimmed());
    println!("{}", "Replacing binary...".dimmed());

    replace_current_exe("creeper_cli")?;
    Ok(())
}

fn replace_current_exe(new_exe_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;

    if OS == "windows" {
        return Err("Replacement is handled by batch script.".into());
    } else {
        fs::rename(new_exe_path, current_exe)?;
    }

    println!("{}", "Binary successfully replaced!".dimmed());
    println!("{}", "Successfully updated CreeperCLI-2! 👍".bright_green());

    Ok(())
}

/*
pub fn update_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to update!");
    print!("{}", "Downloading zip file... ".dimmed());

    let exe_path = env::current_exe()?;
    let mut file_path = exe_path.clone();
    file_path.pop();
    file_path.push("creeper_cli_downloaded.zip");

    fs::rename(&exe_path, "delete_this")?;

    println!("{}", file_path.display().to_string().yellow());

    let file_stem = match OS {
        "windows" => "pc-windows-msvc.zip",
        "linux" => "unknown-linux-gnu.tar.gz",
        "macos" => "apple-darwin.tar.gz",
        _ => {
            println!("{}", "Your operating system is incompatible!".red());
            "error"
        }
    };
    if file_stem == "error" {
        return Ok(())
    }

    let url = format!(
        "https://github.com/creepersaur/CreeperCLI-2/releases/latest/download/creeper_cli-x86_64-{}",
        file_stem
    );
    let mut response = reqwest::blocking::get(url)?;

    let mut dest = File::create(&exe_path)?;
    copy(&mut response, &mut dest)?;

    println!("{}", "Download completed successfully".dimmed());
    print!("{}", "Extracting zip file... ".dimmed());

    let zip_file = File::open(&exe_path)?;
    let mut zip = ZipArchive::new(zip_file)?;
    zip.extract(Path::new(""))?;

    println!("{}", "Zip extraction successful.".dimmed());
    print!("{}", "Deleting zip file... ".dimmed());

    // fs::remove_file(&exe_path)?;
    fs::remove_file("delete_this")?;

    println!("{}", "Deleted zip file.".dimmed());

    println!("{}", "CREEPERCLI UPDATED SUCCESSFULLY! 👍".bright_green());
    Ok(())
}

*/
