use std::{env, fs, process::Command};
use colored::Colorize;

pub fn update_cli() {
    println!("{}", "Attempting to update...".dimmed());

    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let new_exe_name = current_exe.with_file_name("temp_creeper_cli");
    fs::rename(&current_exe, &new_exe_name).expect("Failed to rename the executable");

    let install_file = match cfg!(target_os = "linux") {
        true => "creeper_cli",
        _ => "creeper_cli.exe"
    };

    let output = Command::new("powershell")
            .args(&[
                "-Command",
                format!(
                    "curl https://github.com/creepersaur/CreeperCLI-2/releases/latest/download/{} -o \"{}\"",
                    install_file,
                    current_exe.to_str().unwrap()
                ).as_str()
            ]).spawn().expect("Failed to execute process.");

    // Run the update script
    if let Err(output) = output.wait_with_output() {
        let error = output.to_string();
        eprintln!("{} {}", "Error Failed to update ".red(), "`creeper_cli.exe`".purple());
        eprintln!("[INFO]: {}", error.dimmed());
    } else {
        println!("{}", "Succesfully updated `creeper_cli.exe` !!!".green());
    }
    
    Command::new("powershell")
        .args(&[
            "-Command",
            format!(
                "del {}",
                new_exe_name.display()
            ).as_str()
        ]).spawn().expect("Failed to delete temp_creeper_cli");

    // sleep(Duration::from_secs(1));
    std::process::exit(0);
}

/*fn gpt() {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let new_exe_name = current_exe.with_file_name("temp_creeper_cli.exe");

    fs::rename(&current_exe, &new_exe_name).expect("Failed to rename the executable");

    let update_script = current_exe.with_file_name("update_script.bat");
    fs::write(
        &update_script,
        format!(
            r#"@echo off
            :: Wait for the original exe to close
            timeout /t 2 /nobreak
            :: Download the new version (this can be a curl command or similar)
            curl https://github.com/creepersaur/CreeperCLI-2/releases/latest/download/creeper_cli.exe -o "{0}"
            :: Restart the new executable
            :: Delete the temp file
            del "{1}"

            :: Schedule deletion of this batch file
            call :DeleteSelf

            :: Exit the batch file
            exit /b

            :DeleteSelf
            :: Create a temporary batch file to handle the deletion
            echo @echo off > "%temp%\delete_self.bat"
            echo timeout /t 2 /nobreak >> "%temp%\delete_self.bat"
            echo del "%~f0" >> "%temp%\delete_self.bat"
            echo del "%~dpnx0" >> "%temp%\delete_self.bat"

            :: Run the temporary batch file
            start "" "%temp%\delete_self.bat"

            :: Exit the script
            exit /b
        "#,
            current_exe.display(),
            new_exe_name.display()
        ),
    )
    .expect("Failed to create update script");
}*/