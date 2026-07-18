use std::collections::HashMap;
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

    #[serde(default)]
    #[structly(nested)]
    pub categories: Vec<CommitCategory>,

    pub scopes: Option<HashMap<String, String>>,
}

#[derive(Structly, Deserialize)]
pub struct VersionLocation {
    /// Type of file
    pub r#type: String,
    /// Relative location to the project root of the specified file
    pub location: String,
    /// Useful for more generic file parsers to handle directly unsupported values.
    /// For regex, this would be the regex to select the text to replace.
    /// All the methods here may not be implemented. Please raise an issue if one mentioned sounds good.
    pub reference: Option<String>,
}

#[derive(Structly, Debug, Deserialize)]
pub struct CommitCategory {
    pub title: String,

    #[serde(default)]
    pub tags: Vec<String>,
    /// Will just stop it showing in the release logs.
    /// Though useful for outlining that there are other commit types supported on the project.
    #[serde(default)]
    pub hidden: bool,
}


#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TriggerType {
    PR,
    Manual
}

pub(crate) fn load_from_file(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
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
