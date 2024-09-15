use colored::Colorize;
use curl::easy::{Easy, WriteError};
use std::fs::File;
use std::io::{copy, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use zip::ZipArchive;

#[cfg(target_os = "windows")]
static OS: &str = "windows";
#[cfg(target_os = "linux")]
static OS: &str = "linux";
#[cfg(target_os = "macos")]
static OS: &str = "macos";

pub fn update_cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Attempting to update...".dimmed());

    let current_exe = env::current_exe()?;
    let current_exe_name = current_exe.file_name().unwrap().to_str().unwrap();
    let output_dir = current_exe.parent().unwrap();
    let temp_dir = env::temp_dir();

    let install_file = match OS {
        "windows" => "creeper_cli-x86_64-pc-windows-msvc.zip",
        "linux" => "creeper_cli-x86_64-unknown-linux-gnu.tar.gz",
        "macos" => "creeper_cli-x86_64-apple-darwin.tar.gz",
        _ => return Err("Unsupported operating system".into()),
    };

    let download_url = format!(
        "https://github.com/creepersaur/CreeperCLI-2/releases/latest/download/{}",
        install_file
    );

    println!("{download_url}");

    let temp_zip_path = temp_dir.join(install_file);
    let bytes_downloaded = download_zip(download_url, temp_zip_path.to_str().unwrap())?;
    println!(
        "Download successful: {} ({} bytes)",
        temp_zip_path.display(),
        bytes_downloaded
    );

    let file_size = verify_file(&temp_zip_path)?;
    println!("File verification successful. Size: {} {}", file_size.to_string().purple(), "bytes".purple());

    let temp_exe_path = temp_dir.join("creeper_cli.exe");
    unzip_file(temp_zip_path.to_str().unwrap(), temp_dir.to_str().unwrap())?;
    println!("{}", "Extraction successful".dimmed());

    if !temp_exe_path.exists() {
        return Err("Extracted executable not found".into());
    }

    let script_path = output_dir.join("update_creeper_cli.ps1");
    let update_script = create_update_script(
        &current_exe,
        &temp_exe_path,
        &output_dir.join(current_exe_name),
        &script_path
    );

    fs::write(&script_path, update_script)?;

    println!("{}", "Update prepared successfully.".green());
    println!("{}", "Finalizing update...".dimmed());

    let powershell_output = Command::new("powershell")
        .args(&[
            "-Command",
            format!(
                "powershell -ExecutionPolicy Bypass -File \"{}\"",
                script_path.display()
            )
            .as_str(),
        ])
        .spawn();

    if let Err(output) = powershell_output {
        eprintln!(
            "{} {}\n{}",
            "Error".red(),
            "Failed to execute powershell Script.",
            output.to_string()
        );
        return Ok(());
    }

    println!("{}", "Update was successful!!!".green());

    std::process::exit(0);
}

fn verify_file(path: &PathBuf) -> Result<u64, std::io::Error> {
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len();

    if file_size == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Downloaded file is empty",
        ));
    }

    let mut file = File::open(path)?;
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;

    // Check if it's a zip file (starts with PK\x03\x04)
    if &buffer != b"PK\x03\x04" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File is not a valid zip archive",
        ));
    }

    Ok(file_size)
}

fn download_zip(url: String, output_path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut easy = Easy::new();
    easy.url(&url)?;
    easy.follow_location(true)?; // Follow redirects if any

    let mut file = File::create(output_path)?;
    let mut bytes_downloaded = 0;

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| match file.write_all(data) {
            Ok(_) => {
                bytes_downloaded += data.len() as u64;
                Ok(data.len())
            }
            Err(_) => Err(WriteError::Pause),
        })?;
        transfer.perform()?;
    }

    let response_code = easy.response_code()?;
    if response_code != 200 {
        return Err(format!("HTTP error: {}", response_code).into());
    }

    if bytes_downloaded == 0 {
        return Err("No data downloaded".into());
    }

    Ok(bytes_downloaded)
}

fn unzip_file(zip_path: &str, extract_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = Path::new(extract_path).join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent)?;
                }
            }
            let mut outfile = File::create(&out_path)?;
            copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

fn create_update_script(current_exe: &PathBuf, temp_exe: &PathBuf, target_exe: &PathBuf, script_path: &PathBuf) -> String {
    format!(
        r#"
$ErrorActionPreference = "Stop"
$currentExe = "{}"
$tempExe = "{}"
$targetExe = "{}"

Write-Host "Waiting for current process to exit..."
Start-Sleep -Seconds 2

try {{
    if (Test-Path $currentExe) {{
        Remove-Item $currentExe -Force
        Write-Host "Removed old executable."
    }}
    
    Move-Item $tempExe $targetExe -Force
    Write-Host "Moved new executable to target location."
    
    if (Test-Path $targetExe) {{
        Write-Host "Update successful!"
    }} else {{
        throw "Failed to move new executable to target location."
    }}
}} catch {{
    Write-Host "Error during update: $_"
    exit 1
}}

Write-Host "Update process completed."
del {}
Stop-Process -Id $PID
exit
"#,
        current_exe.display(),
        temp_exe.display(),
        target_exe.display(),
        script_path.display()
    )
}
