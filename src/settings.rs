use std::fs;
use toml::Table;

pub fn get_settings(cwd: &str) -> Result<Table, ()> {
    if let Ok(creeper_toml) = fs::read_to_string(format!("{cwd}//creeper.toml")) {
        if let Ok(settings) = creeper_toml.parse::<Table>() {
            return Ok(settings)
        } else {
            return Err(())
        }
    }

    Err(())
}