use log::debug;
use serde::{Deserialize, Serialize};

static APP: &'static str = "condict";

#[derive(Serialize, Deserialize)]
pub struct CondictConfig {
    pub db_url: String,
}

impl CondictConfig {
    pub fn load() -> anyhow::Result<Self> {
        let location = confy::get_configuration_file_path(APP, Some("config"))?;
        let location = location.display();
        debug!("reading config from {location}");
        let conf = confy::load::<CondictConfig>(APP, Some("config"))?;
        Ok(conf)
    }
}

impl Default for CondictConfig {
    fn default() -> Self {
        Self {
            db_url: "sqlite://:memory:".to_string(),
        }
    }
}
