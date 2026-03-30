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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("error loading config")]
    Config(#[from] confy::ConfyError),

    #[error("database migration error")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("password hashing error: {0}")]
    Hashing(String), // argonautica::Error does not implement std::error::Error

    #[error("error parsing IP address")]
    AddrParse(#[from] std::net::AddrParseError),

    #[error("I/O error")]
    IO(#[from] std::io::Error),
}

impl From<argonautica::Error> for Error {
    fn from(value: argonautica::Error) -> Self {
        Self::Hashing(value.to_string())
    }
}
