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

use dhesinta::config::Config;
use dhesinta::{api, state};

mod app;
use app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let config = Config::load()?;
    let state = state::State::new(config).await?;
    let app = App::new(state);
    app.serve().await?;
    Ok(())
}
