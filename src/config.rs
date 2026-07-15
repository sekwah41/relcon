use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structly::Structly;
use serde::Deserialize;

const CONFIG_FILE: &str = "relcon.toml";

#[derive(Structly, Deserialize)]
pub struct Config {
    #[structly(name = "Release Trigger")]
    pub release_trigger: TriggerType,
}


#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TriggerType {
    PR,
    Manual
}

pub(crate) fn load_from_file(path: PathBuf) -> Result<Config, Box<dyn Error>> {
    log::debug!("Loading config");

    let config_loc = path.join(CONFIG_FILE);

    log::trace!("Attempting to load from: {:?}", config_loc);

    if !config_loc.exists() {
        log::error!("Expected file at: {:?}", config_loc);
    }

    let file_content = fs::read_to_string(&config_loc).map_err(|e| {
        log::error!("Failed to read file at {:?}: {}", config_loc, e);
        e
    })?;

    let config: Config = toml::from_str(&file_content).map_err(|e| {
        log::error!("Failed to parse toml file at {:?}", config_loc);
        log::error!("{}", e);
        e
    })?;

    Ok(config)
}