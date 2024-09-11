use std::fs;
use toml::Table;
use super::CWD;

pub fn get_settings() -> Result<Table, ()> {
    let toml_path = CWD.join("creeper.toml");
    
    if let Ok(creeper_toml) = fs::read_to_string(toml_path) {
        if let Ok(settings) = creeper_toml.parse::<Table>() {
            return Ok(settings)
        } else {
            return Err(())
        }
    }

    Err(())
}