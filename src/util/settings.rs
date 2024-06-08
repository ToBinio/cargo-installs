use color_eyre::eyre;
use home::cargo_home;
use serde::{Deserialize, Serialize};
use std::fs;

pub fn settings() -> eyre::Result<Settings> {
    let path = cargo_home()?.join(".cargo-installs.toml");

    let settings: Settings = if let Ok(content) = fs::read_to_string(path) {
        toml::from_str(&content)?
    } else {
        Settings::default()
    };

    Ok(settings)
}

pub fn save_settings(settings: &Settings) -> eyre::Result<()> {
    let path = cargo_home()?.join(".cargo-installs.toml");

    fs::write(path, toml::to_string_pretty(settings)?)?;

    Ok(())
}

#[derive(Deserialize, Serialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub blacklist: Vec<String>,
}
