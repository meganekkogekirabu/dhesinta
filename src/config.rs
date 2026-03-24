/* Copyright (C) 2026  Madeleine Choi
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use log::debug;
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};

static APP: &'static str = "dhesinta";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub secret_key: String,
    pub net: NetConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetConfig {
    pub hostname: String,
    pub port: u16,
}

impl Config {
    pub fn load() -> crate::Result<Self> {
        let location = confy::get_configuration_file_path(APP, Some("config"))?;
        let location = location.display();
        debug!("reading config from {location}");
        let conf = confy::load::<Config>(APP, Some("config"))?;
        Ok(conf)
    }
}

impl Default for Config {
    fn default() -> Self {
        let secret_key = Alphanumeric.sample_string(&mut rand::rng(), 32);

        Self {
            db_url: "sqlite://:memory:".to_string(),
            secret_key,
            net: NetConfig {
                hostname: "0.0.0.0".into(),
                port: 3000,
            },
        }
    }
}
