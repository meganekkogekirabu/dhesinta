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

use std::io::ErrorKind;

use log::debug;
use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

static PATH: &'static str = "./config.toml";

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
    pub async fn store(&self) -> crate::Result<()> {
        let conf = toml::to_string(self)?;
        tokio::fs::write(PATH, conf).await?;
        Ok(())
    }

    pub async fn load() -> crate::Result<Self> {
        debug!("reading config from {PATH}");
        let file = tokio::fs::File::open(PATH).await;

        let conf = match file {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf).await?;
                toml::from_str(&buf)?
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    let conf = Config::default();
                    conf.store().await?;
                    conf
                }
                e => {
                    panic!("failed to open config: {e}");
                }
            },
        };

        Ok(conf)
    }
}

impl Default for Config {
    fn default() -> Self {
        let secret_key = Alphanumeric.sample_string(&mut rand::rng(), 32);

        Self {
            db_url: "sqlite://./db/db.sqlite3".to_string(),
            secret_key,
            net: NetConfig {
                hostname: "0.0.0.0".into(),
                port: 3000,
            },
        }
    }
}
