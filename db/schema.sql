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

CREATE TABLE IF NOT EXISTS dictionaries (
    id CHAR(21) PRIMARY KEY NOT NULL UNIQUE,
    owner_id CHAR(21) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    visibility VARCHAR(8) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS entries (
    id CHAR(21) PRIMARY KEY NOT NULL UNIQUE,
    dictionary_id CHAR(21) NOT NULL,
    word TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (dictionary_id) REFERENCES dictionaries(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS entry_fields (
    entry_id CHAR(21) NOT NULL,
    field_key TEXT NOT NULL,
    field_value TEXT NOT NULL,
    PRIMARY KEY (entry_id, field_key),
    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE
);
