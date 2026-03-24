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

CREATE TABLE _entries (
    id CHAR(21) PRIMARY KEY NOT NULL UNIQUE,
    dictionary_id CHAR(21) NOT NULL,
    owner_id CHAR(21) NOT NULL,
    word TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (dictionary_id) REFERENCES dictionaries(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

INSERT INTO _entries (id, dictionary_id, word, created_at, updated_at)
SELECT * FROM entries;

UPDATE _entries
SET owner_id = (
    SELECT d.owner_id
    FROM dictionaries d
    WHERE d.id = _entries.dictionary_id
);

DROP TABLE entries;

ALTER TABLE _entries RENAME TO entries;
